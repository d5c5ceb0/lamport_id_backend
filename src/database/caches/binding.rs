use crate::{
    common::error::AppResult,
    database::{
        dals::twitter_binding::*,
        Storage,
    },
    helpers::redis_cache::*,
};

pub type TwitterBinding = TwitterBindingModel;

const TWITTER_BINDING_KEY : &str = "twitter_binding";

impl Storage {
    //create twitter binding
    pub async fn binding_twitter(
        &self,
        binding: &TwitterBinding,
    ) -> AppResult<TwitterBinding> {
        let cache = RedisClient::from(self.redis.clone());

        let binding = self.create_twitter_binding(binding.clone()).await?;

        cache.invalidate_cache(format!("{}:{}", TWITTER_BINDING_KEY, binding.lamport_id).as_str()).await?;

        Ok(binding)
    }

    pub async fn update_twitter_stats(
        &self,
        lamport_id: &str,
        retweet: i32,
        mention: i32,
        comment: i32,
        quote: i32,
    ) -> AppResult<TwitterBinding> {
        let cache = RedisClient::from(self.redis.clone());

        let binding = self.update_twitter_binding_by_lamport_id(
            lamport_id,
            retweet,
            mention,
            comment,
            quote,
        ).await?;

        cache.invalidate_cache(format!("{}:{}", TWITTER_BINDING_KEY, binding.lamport_id).as_str()).await?;

        Ok(binding)
    }



    //get twitter binding by user id
    pub async fn get_twitter_binding_by_user_id(&self, user_id: &str) -> AppResult<TwitterBinding> {
        let cache = RedisClient::from(self.redis.clone());


        let key = format!("{}:{}", TWITTER_BINDING_KEY, user_id);

        if let Ok(data) = cache.get_data(&key).await {
            tracing::info!("get data from cache:{:?}", data);
            return Ok(data);
        }

        let binding= self.get_twitter_binding_by_lamport_id(user_id).await?;

        cache.set_data(&key, &binding).await?;

        Ok(binding)
    }

}

