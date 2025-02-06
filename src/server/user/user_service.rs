use super::user_message::*;
use crate::{
    app::SharedState, 
    common::{
        error::{AppResult, AppError}, 
        consts
    },
    server::{
        auth::auth_message::*,
        events::events_message::Event
    },
    database::dals::telegram_binding,
    database::services::binding,
    nostr,
    helpers::{eip191::verify_signature, redis_cache::*},
};
use reqwest::Client;

const TWITTER_AUTH_ENDPOINT: &str = "https://api.x.com/2/oauth2/token";
const TWITTER_API_ENDPOINT: &str = "https://api.x.com/2/users/me";
const DISCORD_CLIENT_ID: &str = "1336590096928866306";
const DISCORD_CLIENT_SECRET: &str = "5DcNW65ua3Av76eQuCn0wdDz4PErna2F";
const DISCORD_AUTH_ENDPOINT: &str = "https://discord.com/api/v10/oauth2/token";
const DISCORD_API_ENDPOINT: &str = "https://discord.com/api/v10/users/@me";
const GITHUB_CLIENT_ID: &str = "Iv23lii7LrglBj8Q0mvv";
const GITHUB_CLIENT_SECRET: &str = "7bd3652c28928d26cad5b9aa04ed12c324a86b91";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_API_URL: &str = "https://api.github.com/user";


pub async fn user_get_user_info(
    state: &SharedState,
    lamport_id: &str,
) -> AppResult<UserResponse> {
    let user = state.store.get_user_by_uid(lamport_id).await?;
    let user_rep = UserResponse::from(user);

    Ok(user_rep)
}

pub async fn user_get_user_count(
    state: &SharedState,
) -> AppResult<u64> {
    let count = state.store.count_total_users().await?;

    Ok(count)
}

pub async fn user_get_user_stats(
    state: &SharedState,
    lamport_id: &str,
) -> AppResult<PointsResponse> {
    let invite_count = state.store.count_invited_users_by_uid(lamport_id) .await?;

    let point = match state.store.get_user_points(lamport_id).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    let energy = match state.store.get_user_power(lamport_id).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    //pub async fn get_user_daily_points(&self, user_uid: &str) -> AppResult<i64> {
    let daily_point = match state.store.get_user_daily_points(lamport_id).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    Ok(PointsResponse{point, invite_count, energy, daily_point})
}

pub async fn user_check_username(
    state: &SharedState,
    username: &str,
) -> AppResult<()> {
    let existing= state.store.is_user_exists_by_username(username).await;
    match existing {
        Ok(true) => {
            Err(AppError::ConflictError("Name is already taken".to_string()))
        }
        Ok(false) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn user_verify_user(
    state: &SharedState,
    lamport_id: &str,
    address: &str,
) -> AppResult<()> {
    //get user by uid
    let verifier = state.store.get_user_by_uid(lamport_id).await?;
    if !verifier.verified {
        return Err(AppError::InputValidateError("verifier is not verified".into()));
    }

    let _user = state.store.update_user(address, lamport_id).await?;

    Ok(())
}

pub async fn user_login(
    state: &SharedState,
    req: LoginRequest,
) -> AppResult<(String, UserResponse)> {
    if cfg!(not(debug_assertions)) {
        let verified= verify_signature(&req.data, &req.sig, &req.data.address)?;
        if !verified {
            return Err(AppError::InvalidSignature);
        }
        tracing::info!("signature verified success");

        let redis_client = RedisClient::from(state.redis.clone());

        if let Ok(nonce) = redis_client.get_nonce(req.data.address.as_str()).await {
            tracing::info!("got nonce: {:?} by key: {:?}", nonce, req.data.address);
            if nonce != req.data.nonce {
                return Err(AppError::InputValidateError("nonce verification error".into()));
            }
        } else {
            tracing::error!("got nonce err: wrong address:{:?} ", req.data.address);
            return Err(AppError::InputValidateError(
                    "nonce is not existing".into(),
            ));
        }

        match redis_client.del_nonce(req.data.address.as_str()).await {
            Ok(_) => {
                tracing::info!("delete nonce success");
            }
            Err(e) => {
                tracing::error!("delete nonce err: {:?}", e);
            }
        }
    }

    let user = state.store.get_user_by_address(&req.data.address).await?;
    let secret = state.jwt_handler.clone();
    let token: String =
        secret.create_token(&user.lamport_id, &user.name, &user.user_name);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    Ok((token, UserResponse::from(user)))
}

pub async fn user_register(
    state: &SharedState,
    req: RegisterRequest,
) -> AppResult<(String, UserResponse)> {
    if cfg!(not(debug_assertions)) {
        let verified= verify_signature(&req.data, &req.sig, &req.data.address)?;
        if !verified {
            return Err(AppError::InvalidSignature);
        }
        tracing::info!("signature verified success");

        let redis_client = RedisClient::from(state.redis.clone());
        if let Ok(nonce) = redis_client.get_nonce(req.data.address.as_str()).await {
            tracing::info!("got nonce: {:?} by key: {:?}", nonce, req.data.address);
            if nonce != req.data.nonce {
                return Err(AppError::InputValidateError("nonce verification error".into()));
            }
        } else {
            tracing::error!("got nonce err: wrong address:{:?} ", req.data.address);
            return Err(AppError::InputValidateError(
                    "nonce is not existing".into(),
            ));
        }

        match redis_client.del_nonce(req.data.address.as_str()).await {
            Ok(_) => {
                tracing::info!("delete nonce success");
            }
            Err(e) => {
                tracing::error!("delete nonce err: {:?}", e);
            }
        }
    }

    let user_info: RegisterRequest = req.clone();

    let created_user = if state
        .store
        .is_user_exists_by_address(user_info.data.address.as_ref())
        .await?
    {
        state
            .store
            .get_user_by_address(user_info.data.address.as_ref())
            .await?
    } else {
        let user: User = User::from(user_info.clone());

        let created_user = match state.store.create_user(user.into()).await {
            Ok(u) => u,
            Err(AppError::UserExisted(_)) => {
                tracing::info!("user has already existed, log in");
                state
                    .store
                    .get_user_by_username(user_info.data.user_name.as_ref())
                    .await?
            }
            Err(e) => return Err(e),
        };

        state
            .store
            .create_energy(created_user.clone().lamport_id, consts::ENERGY_REGISTER, consts::ENERGY_REGISTER_VALUE)
            .await?;

        if let Some(invited_by)  = created_user.invited_by.as_deref() {
            let inviter = state.store.get_inviter_by_code(invited_by).await?;

            //award point
            state
                .store
                .award_points(inviter.lamport_id.clone(), consts::POINTS_INVITE, consts::POINTS_INVITE_VALUE, consts::INVITE_TWITTER_CHANNEL)
                .await?;

            //consume energy 
            state
                .store
                .create_energy(inviter.lamport_id, consts::ENERGY_INVITE, consts::ENERGY_INVITE_VALUE)
                .await?;

        }

        let queue = state.queue.clone();

        let e = Event {
            event_id: uuid::Uuid::new_v4().to_string(),
            lamport_id: created_user.lamport_id.clone(),
            event_type: consts::EVENT_TYPE_REGISTER.to_string(),
            content: "First time using HetuVerse to generate Lamper ID".to_string(),
            created_at: chrono::Utc::now(),
        };
        queue.add_queue_req_ex(consts::EVENT_TOPIC, e).await?;

        queue.add_queue_req_ex(consts::NOSTR_TOPIC, nostr::LamportBinding::new_kind2322(state.nclient.get_pub_key(),created_user.lamport_id.as_str(), created_user.address.as_str(),"")).await?;

        tracing::info!("[auth_token] database  user info: {:?}", created_user);
        created_user
    };

    let secret = state.jwt_handler.clone();
    let token: String =
        secret.create_token(&created_user.lamport_id, &created_user.name, &created_user.user_name);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    Ok((token, UserResponse::from(created_user)))
}

pub async fn user_binding_twitter(
    state: &SharedState,
    lamport_id: String,
    params: OAuthParams,
) -> AppResult<BindingTwitterResponse> {
    if let Ok(t) = state.store.get_twitter_binding_by_user_id(lamport_id.as_str()).await {
        return Ok(BindingTwitterResponse::from(t));
    }

    let token_params = [
        ("code", params.clone().code.unwrap()),
        ("grant_type", "authorization_code".into()),
        ("client_id", state.config.auth.client_id.clone()),
        ("redirect_uri", params.clone().redirect_uri.unwrap()),
        ("code_verifier", "challenge".into()),
    ];

    tracing::info!("[auth_token] exchange code: {:?}", token_params);


    let token: ExchangeTokenRespose = OauthRequest::new().exchange_code(TWITTER_AUTH_ENDPOINT, token_params.to_vec(), Some(vec![("Content-Type", "application/x-www-form-urlencoded")]), None).await?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token.clone();

    let user_info: OauthUserInfo = OauthRequest::new()
        .get_user_info(TWITTER_API_ENDPOINT, &access_token, None, Some(vec![("user.fields", "profile_image_url")]))
        .await?;

    tracing::info!("[auth_token] get user info: {:?}", user_info);

    let binding  = binding::TwitterBinding::new(
        lamport_id.clone(),
        user_info.data.id.clone(),
        user_info.data.name.clone(),
        user_info.data.username.clone(),
        user_info.data.profile_image_url.clone(),
        token.access_token.clone(),
        token.refresh_token.clone(),
        token.token_type.clone(),
        token.scope.clone(),
    );

    let created_binding= match state.store.binding_twitter(&binding).await {
        Ok(u) => u,
        Err(AppError::UserExisted(_)) => {
            tracing::info!("user has already existed, log in");
            state
                .store
                .get_twitter_binding_by_user_id(lamport_id.as_str())
                .await?
        }
        Err(e) => return Err(e),
    };

    //award point
    state
        .store
        .award_points(lamport_id.clone(), consts::POINTS_BINDING, consts::POINTS_BINDING_VALUE, "twitter")
        .await?;

    //consume energy 
    state
        .store
        .create_energy(lamport_id.clone(), consts::ENERGY_BINDING, consts::ENERGY_BINDING_VALUE)
        .await?;


    tracing::info!("[auth_token] database  user info: {:?}", created_binding);

    //pub fn new_kind2321(pubkey: PublicKey, lamport_id: &str, twitter: &str) -> Self {
    state.queue.add_queue_req_ex(consts::NOSTR_TOPIC, nostr::LamportBinding::new_kind2321(
        state.nclient.get_pub_key(),
        lamport_id.as_str(),
        created_binding.user_name.as_str(),
    )).await?;

    Ok(BindingTwitterResponse::from(created_binding))
}


pub async fn user_get_twitter_binding(
    state: &SharedState,
    lamport_id: &str,
) -> AppResult<BindingTwitterResponse> {
    match state.store.get_twitter_binding_by_user_id(lamport_id).await {
        Ok(binding) => Ok(BindingTwitterResponse::from(binding)),
        Err(e) => Err(e),
    }
}

pub async fn user_binding_telegram(
    state: &SharedState,
    params: &TelegramParams,
) -> AppResult<BindingTelegramResponse> {
    let redis_client = RedisClient::from(state.redis.clone());
    let lamport_id = if let Ok(token) = redis_client.get_lamportid_token(params.token.as_str()).await {
        tracing::info!("got token: {:?} by key: {:?}", token, params.token);
        token
    } else {
        tracing::error!("got token err: wrong token:{:?} ", params.token);
        return Err(AppError::InputValidateError(
                "token is not existing".into(),
        ));
    };

    match redis_client.del_lamportid_token(params.token.as_str()).await {
        Ok(_) => {
            tracing::info!("delete token success");
        }
        Err(e) => {
            tracing::error!("delete token err: {:?}", e);
        }
    }

    //save to db
    let binding = match state.store.create_telegram_binding(telegram_binding::TelegramBindingModel::new(
        lamport_id.clone(),
        params.user_id.clone(),
    )).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("user has already existed, {:?}", e);
            state
                .store
                .get_telegram_binding_by_lamport_id(lamport_id.as_str())
                .await?
        },
    };

    //award point
    state
        .store
        .award_points(lamport_id.clone(), consts::POINTS_BINDING, consts::POINTS_BINDING_VALUE, "telegram")
        .await?;

    //consume energy 
    state
        .store
        .create_energy(lamport_id.clone(), consts::ENERGY_BINDING, consts::ENERGY_BINDING_VALUE)
        .await?;

    Ok(BindingTelegramResponse::from(binding))
}

pub async fn user_binding_discord(
    state: &SharedState,
    lamport_id: &str,
    params: &OAuthParams,
) -> AppResult<OauthDiscordInfo> {

    let token_params = [
        ("code", params.clone().code.unwrap()),
        ("grant_type", "authorization_code".into()),
        ("redirect_uri", params.clone().redirect_uri.unwrap()),
    ];

    let token: TokenResponse = OauthRequest::new()
        .exchange_code(DISCORD_AUTH_ENDPOINT, token_params.to_vec(), Some(vec![("Content-Type", "application/x-www-form-urlencoded")]), Some((DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET)))
        .await?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token.clone();
    let user_info: OauthDiscordInfo = OauthRequest::new()
        .get_user_info(DISCORD_API_ENDPOINT, &access_token, None, None)
        .await?;

    tracing::info!("[auth_token] get user info: {:?}", user_info);


    Ok(user_info)
}


pub async fn user_binding_github(
    state: &SharedState,
    lamport_id: &str,
    params: &OAuthParams,
) -> AppResult<GitHubUser> {
    let token_params = [
        ("code", params.clone().code.unwrap()),
        ("client_id", GITHUB_CLIENT_ID.into()),
        ("client_secret", GITHUB_CLIENT_SECRET.into()),
    ];

    let token :TokenResponse = OauthRequest::new()
        .exchange_code(GITHUB_TOKEN_URL, token_params.to_vec(), Some(vec![("Accept", "application/json")]), None)
        .await?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token.clone();

    let user_info: GitHubUser = OauthRequest::new()
        .get_user_info(GITHUB_API_URL, &access_token, Some(vec![("User-Agent","lamportid")]), None)
        .await?;
    tracing::info!("[auth_token] get user info: {:?}", user_info);

    Ok(user_info)
}



#[derive(Debug)]
pub struct OauthRequest(reqwest::Client);

impl Default for OauthRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl OauthRequest {
    pub fn new() -> Self {
        OauthRequest(reqwest::Client::new())
    }

    pub async fn get_user_info<T>(
        &self, 
        url: &str,
        access_token: &str,
        //user_agent: Option<String>,
        header_fields: Option<Vec<(&str, &str)>>,
        query_fields: Option<Vec<(&str, &str)>>
    ) -> AppResult<T> 
    where
        T: for<'de> serde::Deserialize<'de> 
    {
        let mut request= Client::new()
            .get(url)
            .bearer_auth(access_token);

        if let Some(fields) = query_fields {
            request = request.query(&fields);
        }

        if let Some(fields) = header_fields {
            for (k, v) in fields {
                request = request.header(k, v);
            }
        }


        let response = request.send().await.map_err(|_e| AppError::RequestError("failed to get user info".to_string()))?;

        if !response.status().is_success() {
            tracing::error!(" get user error");
            return Err(AppError::RequestError(
                    "non user info in response".to_string(),
            ));
        }

        let user_info: T = response
            .json()
            .await
            .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;


        Ok(user_info)
    }

    //.header("Content-Type", "application/x-www-form-urlencoded")
    //.header("Accept", "application/json")
    //.basic_auth(DISCORD_CLIENT_ID, Some(DISCORD_CLIENT_SECRET))
    pub async fn exchange_code<T>(
        &self, 
        url: &str,
        params: Vec<(&str, String)>,
        header_fields: Option<Vec<(&str, &str)>>,
        basci_auth: Option<(&str, &str)>
    ) -> AppResult<T> 
    where
        T: for<'de> serde::Deserialize<'de> 
    {
        let mut request= self.0
            .post(url);

        if let Some(fields) = header_fields {
            for (k, v) in fields {
                request = request.header(k, v);
            }
        }

        if let Some((k, v)) = basci_auth {
            request = request.basic_auth(k, Some(v));
        }

        let response = request.form(&params).send()
            .await
            .map_err(|_e| AppError::RequestError("failed to exchange code".to_string()))?;


        if !response.status().is_success() {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            return Err(AppError::RequestError(format!(
                        "Failed to get token. Status: Error: {}", 
                        error_message
            )));
        }

        let token: T = response
            .json()
            .await
            .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

        Ok(token)
    }
}
