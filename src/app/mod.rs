use crate::{
    common::{
        config::Config,
        consts,
        error::{AppError, AppResult},
    },
    database,
    helpers::google_auth,
    nostr,
    queue::msg_queue::{MessageQueue, RedisMessage, RedisStreamPool},
    server::{http_server_start, middlewares::jwt::jwt_handler, events::events_message::Event},
};
// use ::nostr::event::Kind;
use oauth2::basic::BasicClient;
use std::ops::Deref;
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub store: database::Storage,
    pub jwt_handler: jwt_handler::JwtHandler,
    pub oauth: BasicClient,
    pub redis: redis::Client,
    pub queue: RedisStreamPool,
    pub nclient: nostr::NostrClient,
}

impl AppState {
    pub async fn new(path: PathBuf) -> Self {
        let config = Config::load_config(path).unwrap();
        let redis_client = redis::Client::open(config.redis.redis_url.as_str()).unwrap();

        let store = database::Storage::new(config.database.clone(), redis_client.clone()).await;

        let secret = consts::JWT_SECRET_KEY.to_string();
        let jwt_handler = jwt_handler::JwtHandler { secret };

        Self {
            config: config.clone(),
            store,
            jwt_handler,
            oauth: google_auth::oauth_client(config.auth),
            redis: redis_client,
            queue: RedisStreamPool::new(config.redis.redis_url.as_str())
                .await
                .unwrap(),
            nclient: nostr::NostrClient::new(
                config.nostr.priv_key.as_str(),
                Some(config.nostr.ws_url.as_str()),
            )
            .await
            .unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SharedState(pub Arc<AppState>);

impl Deref for SharedState {
    type Target = AppState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SharedState {
    pub async fn new(path: PathBuf) -> Self {
        let state = AppState::new(path).await;
        SharedState(Arc::new(state))
    }

    pub async fn run(&self) -> AppResult<()> {
        let nclient = self.nclient.clone();
        let queue = self.queue.clone();

        //let queue_topic = self.config.redis.topic.clone();
        let queue_topic = consts::NOSTR_TOPIC;
        tokio::spawn(async move {
            loop {
                match queue.consume(queue_topic).await {
                    Ok(msgs) => {
                        for (_k, m) in msgs.iter().enumerate() {
                            //Deserialize data
                            let msg: nostr::LamportBinding = match serde_json::from_str(m.data.as_str()) {
                                    Ok(parsed) => parsed,
                                    Err(e) => {
                                        tracing::error!(
                                            "Failed to parse message: {}, error: {:?}",
                                            m.data,
                                            e
                                        );
                                        if let Err(e) = queue.acknowledge(queue_topic, &m.id).await
                                        {
                                            tracing::error!(
                                                "Failed to acknowledge message: {}, error: {:?}",
                                                m.id,
                                                e
                                            );
                                        }
                                        continue;
                                    }
                                };

                            tracing::info!("Received message: {:?}", msg);
                            nclient.sign_and_send(&msg).await.unwrap();

                            // ack
                            if let Err(e) = queue.acknowledge(queue_topic, &m.id).await {
                                tracing::error!(
                                    "Failed to acknowledge message: {}, error: {:?}",
                                    m.id,
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to consume messages from queue: {:?}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        });

        let timeline_queue = self.queue.clone();
        let timeline_topic = consts::EVENT_TOPIC;
        let store = self.store.clone();
        tokio::spawn(async move {
            loop {
                match timeline_queue.consume(timeline_topic).await {
                    Ok(msgs) => {
                        for (_k, m) in msgs.iter().enumerate() {
                            //Deserialize data
                            let  msg: Event = match serde_json::from_str(m.data.as_str()) {
                                Ok(parsed) => parsed,
                                Err(e) => {
                                    tracing::error!("Failed to parse message: {}, error: {:?}", m.data, e);
                                    if let Err(e) = timeline_queue.acknowledge(timeline_topic, &m.id).await {
                                        tracing::error!(
                                            "Failed to acknowledge message: {}, error: {:?}",
                                            m.id,
                                            e
                                        );
                                    }
                                    continue;
                                }
                            };

                            store.create_event(msg.lamport_id, msg.event_type, msg.content).await.unwrap();

                            // ack
                            if let Err(e) = timeline_queue.acknowledge(timeline_topic, &m.id).await {
                                tracing::error!("Failed to acknowledge message: {}, error: {:?}", m.id, e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to consume messages from queue: {:?}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        });


        http_server_start(self.clone()).await?;

        Ok(())
    }
}

impl RedisStreamPool {
    #[allow(dead_code)]
    pub async fn add_queue_req(&self, topic: &str, id: String, p: serde_json::Value) -> AppResult<()> {
        let redis_msg = match RedisMessage::new((id, p)) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("redis message new error: {:?}", e);
                return Err(AppError::CustomError("redis msg new error".into()));
            }
        };
        tracing::info!("Product message: data={:?}", redis_msg);
        if let Err(e) = self.produce(topic, &redis_msg).await {
            tracing::error!("redis queue produce error: {:?}", e);
            return Err(AppError::CustomError("redis queue produce error".into()));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn add_queue_req_ex(
        &self,
        topic: &str,
        p: impl serde::Serialize,
    ) -> AppResult<()> {
        let redis_msg = match RedisMessage::new(p) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("redis message new error: {:?}", e);
                return Err(AppError::CustomError("redis msg new error".into()));
            }
        };
        tracing::info!("Product message: data={:?}", redis_msg);
        if let Err(e) = self.produce(topic, &redis_msg).await {
            tracing::error!("redis queue produce error: {:?}", e);
            return Err(AppError::CustomError(
                "redis queue produce error".into(),
            ));
        }

        Ok(())
    }
}
