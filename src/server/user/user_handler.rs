use super::user_message::*;
use crate::{
    app::SharedState, 
    common::{
        error::{AppResult, AppError}, 
        consts
    },
    server::{
        middlewares::AuthToken,
        auth::auth_message::*
    },
    database::dals::telegram_binding,
    database::services::binding,
    nostr,
    helpers::redis_cache::*,
};
use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[debug_handler]
pub async fn get_user_info(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let user = state.store.get_user_by_uid(claim.sub.as_ref()).await?;
    let user_rep = UserResponse::from(user);

    Ok(Json(serde_json::json!({
    "result": user_rep
    })))
}

#[debug_handler]
pub async fn get_user_count(
    State(state): State<SharedState>,
    AuthToken(_user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let count = state.store.count_total_users().await?;

    Ok(Json(serde_json::json!({
    "result": CountResponse{count}
    })))
}

#[debug_handler]
pub async fn get_user_stats(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let invite_count = state
        .store
        .count_invited_users_by_uid(claim.sub.as_ref())
        .await?;

    tracing::info!("sub: {:?}", claim.sub);
    let point = match state.store.get_user_points(claim.sub.as_ref()).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    let energy = match state.store.get_user_power(claim.sub.as_ref()).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    //pub async fn get_user_daily_points(&self, user_uid: &str) -> AppResult<i64> {
    let daily_point = match state.store.get_user_daily_points(claim.sub.as_ref()).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    Ok(Json(serde_json::json!({
    "result": PointsResponse{point, invite_count, energy, daily_point}
    })))
}

// post binding_account
#[debug_handler]
pub async fn binding_account(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    if let Ok(t) = state.store.get_twitter_binding_by_user_id(claim.sub.as_str()).await {
        return Ok(Json(serde_json::json!({
            "result": {
                "twitter_info": BindingTwitterResponse::from(t)
            }
        })));
    }

    tracing::info!("[auth_token] get params: {:?}", params);

    params.validate_items()?;

    let client = Client::new();

    let token_params = [
        ("code", params.clone().code.unwrap()),
        ("grant_type", "authorization_code".into()),
        ("client_id", state.config.auth.client_id.clone()),
        ("redirect_uri", params.clone().redirect_uri.unwrap()),
        ("code_verifier", "challenge".into()),
    ];

    tracing::info!("[auth_token] exchange code params: {:?}", token_params);


    let token_response= client
        .post("https://api.x.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&token_params)
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to exchange code".to_string()))?;

    if !token_response.status().is_success() {
        let error_message = token_response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());

        return Err(AppError::RequestError(format!(
                    "Failed to get token. Status: Error: {}", 
                    error_message
        )));
    }

    let token: ExchangeTokenRespose = token_response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token.clone();

    tracing::info!("[auth_token] Access Token: {:?}", access_token);

    let client = Client::new();

    let user_info_response = client
        .get("https://api.x.com/2/users/me")
        .bearer_auth(&access_token)
        .query(&[("user.fields", "profile_image_url")])
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to get user info".to_string()))?;

    tracing::info!("[auth_token] get user info response: {:?}", user_info_response);

    if !user_info_response.status().is_success() {
        return Err(AppError::RequestError(
            "non user info in response".to_string(),
        ));
    }

    let user_info: OauthUserInfo = user_info_response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    tracing::info!("[auth_token] get user info: {:?}", user_info);


    let binding  = binding::TwitterBinding::new(
        claim.sub.clone(),
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
                .get_twitter_binding_by_user_id(claim.sub.as_str())
                .await?
        }
        Err(e) => return Err(e),
    };

    //award point
    state
        .store
        .award_points(claim.sub.clone(), consts::POINTS_BINDING, consts::POINTS_BINDING_VALUE, "twitter")
        .await?;

    //consume energy 
    state
        .store
        .create_energy(claim.sub.clone(), consts::ENERGY_BINDING, consts::ENERGY_BINDING_VALUE)
        .await?;


    tracing::info!("[auth_token] database  user info: {:?}", created_binding);

    //pub fn new_kind2321(pubkey: PublicKey, lamport_id: &str, twitter: &str) -> Self {
    state.queue.add_queue_req_ex(consts::NOSTR_TOPIC, nostr::LamportBinding::new_kind2321(
        state.nclient.get_pub_key(),
        claim.sub.as_str(),
        created_binding.user_name.as_str(),
    )).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "twitter_info": BindingTwitterResponse::from(created_binding)
        }
    })))
}

//struct for get binding twitter and impl from twitter_binding::Model
#[derive(Debug, Serialize, Deserialize)]
pub struct BindingTwitterResponse {
    pub x_id: String,
    pub name: String,
    pub user_name: String,
    pub image_url: String,
}

impl From<binding::TwitterBinding> for BindingTwitterResponse {
    fn from(binding: binding::TwitterBinding) -> Self {
        Self {
            x_id: binding.x_id,
            name: binding.name,
            user_name: binding.user_name,
            image_url: binding.image_url,
        }
    }
}


// get_user_bindings
#[debug_handler]
pub async fn get_user_bindings(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    match state.store.get_twitter_binding_by_user_id(claim.sub.as_str()).await {
        Ok(binding) => Ok(Json(serde_json::json!({
                "result": {
                    "twitter_info": BindingTwitterResponse::from(binding)
                }
            }))),
        Err(AppError::CustomError(_)) => Ok(Json(serde_json::json!({
                "result": {
                    "twitter_info": {}
                }
            }))),
        Err(e) => Err(e),

    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramParams {
    pub user_id: String,
    pub token: String,
}

//telgram binding response
#[derive(Debug, Serialize, Deserialize)]
pub struct BindingTelegramResponse {
    pub lamport_id: String,
    pub user_id: String,
}

impl From<telegram_binding::TelegramBindingModel> for BindingTelegramResponse {
    fn from(binding: telegram_binding::TelegramBindingModel) -> Self {
        Self {
            lamport_id: binding.lamport_id,
            user_id: binding.telegram_id,
        }
    }
}

pub async fn binding_telegram(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Json(params): Json<TelegramParams>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();


    let redis_client = RedisClient::from(state.redis.clone());

    if let Ok(token) = redis_client.get_csrf_token(params.token.as_str()).await {
        tracing::info!("got token: {:?} by key: {:?}", token, params.token);
    } else {
        tracing::error!("got token err: wrong token:{:?} ", params.token);
        return Err(AppError::InputValidateError(
                "token is not existing".into(),
        ));
    }

    match redis_client.del_csrf_token(params.token.as_str()).await {
        Ok(_) => {
            tracing::info!("delete token success");
        }
        Err(e) => {
            tracing::error!("delete token err: {:?}", e);
        }
    }

    //save to db
    let binding = state.store.create_telegram_binding(telegram_binding::TelegramBindingModel::new(
        claim.sub.clone(),
        params.user_id.clone(),
    )).await?;

    //award point
    state
        .store
        .award_points(claim.sub.clone(), consts::POINTS_BINDING, consts::POINTS_BINDING_VALUE, "telegram")
        .await?;

    //consume energy 
    state
        .store
        .create_energy(claim.sub.clone(), consts::ENERGY_BINDING, consts::ENERGY_BINDING_VALUE)
        .await?;

    Ok(Json(serde_json::json!({
        "result": BindingTelegramResponse::from(binding)
    })))
}

const API_ENDPOINT: &str = "https://discord.com/api/v10";
const CLIENT_ID: &str = "1336590096928866306";
const CLIENT_SECRET: &str = "5DcNW65ua3Av76eQuCn0wdDz4PErna2F";

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}
//{\"id\":\"1142062260328603648\",\"username\":\"energetic_dove_64259\",\"avatar\":null,\"discriminator\":\"0\",\"public_flags\":0,\"flags\":0,\"banner\":null,\"accent_color\":null,\"global_name\":\"aa\",\"avatar_decoration_data\":null,\"banner_color\":null,\"clan\":null,\"primary_guild\":null,\"mfa_enabled\":false,\"locale\":\"zh-CN\",\"premium_type\":0,\"email\":\"d5c5ceb0@gmail.com\",\"verified\":true}\
#[derive(Serialize, Deserialize, Debug)]
pub struct OauthDiscordInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: u64,
    pub flags: u64,
    pub banner: Option<String>,
    pub accent_color: Option<String>,
    pub global_name: String,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub clan: Option<String>,
    pub primary_guild: Option<String>,
    pub mfa_enabled: bool,
    pub locale: String,
    pub premium_type: u64,
    pub email: String,
    pub verified: bool,
}

pub async fn binding_discord(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    tracing::info!("[auth_token] get params: {:?}", params);
    //if existed TODO
    //

    let client = Client::new();

    let token_params = [
        ("code", params.clone().code.unwrap()),
        ("grant_type", "authorization_code".into()),
        ("redirect_uri", params.clone().redirect_uri.unwrap()),
    ];

    tracing::info!("[auth_token] exchange code params: {:?}", token_params);


    let token_response= client
        .post(&format!("{}/oauth2/token", API_ENDPOINT))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .form(&token_params)
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to exchange code".to_string()))?;

    if !token_response.status().is_success() {
        let error_message = token_response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());

        return Err(AppError::RequestError(format!(
                    "Failed to get token. Status: Error: {}", 
                    error_message
        )));
    }

    let token: TokenResponse = token_response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token.clone();

    tracing::info!("[auth_token] Access Token: {:?}", access_token);

    ///
    let client = Client::new();

    let user_info_response = client
        .get("https://discord.com/api/v10/users/@me")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to get user info".to_string()))?;

    tracing::info!("[auth_token] get user info response: {:?}", user_info_response);

    if !user_info_response.status().is_success() {
        let error_message =user_info_response 
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        tracing::error!("[auth_token] get user error: {:?}", error_message);
        return Err(AppError::RequestError(
            "non user info in response".to_string(),
        ));
    }

    let user_info: OauthDiscordInfo = user_info_response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    tracing::info!("[auth_token] get user info: {:?}", user_info);


    Ok(Json(serde_json::json!({
        "result": user_info
    })))
}


const GITHUB_CLIENT_ID: &str = "Iv23lii7LrglBj8Q0mvv";
const GITHUB_CLIENT_SECRET: &str = "7bd3652c28928d26cad5b9aa04ed12c324a86b91";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_API_URL: &str = "https://api.github.com/user";

#[derive(Debug, Serialize, Deserialize)]
struct GitHubUser {
    login: String,
    id: u64,
    avatar_url: String,
}

pub async fn binding_github(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    tracing::info!("[auth_token] get params: {:?}", params);
    //if existed TODO
    //

    let client = Client::new();

    let token_params = [
        ("code", params.clone().code.unwrap()),
        ("client_id", GITHUB_CLIENT_ID.into()),
        ("client_secret", GITHUB_CLIENT_SECRET.into()),
    ];

    tracing::info!("[auth_token] exchange code params: {:?}", token_params);

    let token_response= client
        .post(GITHUB_TOKEN_URL)
        .header("Accept", "application/json")
        .form(&token_params)
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to exchange code".to_string()))?;

    if !token_response.status().is_success() {
        let error_message = token_response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());

        return Err(AppError::RequestError(format!(
                    "Failed to get token. Status: Error: {}", 
                    error_message
        )));
    }

    let token: TokenResponse = token_response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token.clone();

    tracing::info!("[auth_token] Access Token: {:?}", access_token);

    let client = Client::new();
    let response = client
        .get(GITHUB_API_URL)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "lamportid")
        .send()
        .await
        .map_err(|_e| AppError::RequestError("failed to get user info".to_string()))?;

    //let user_info = response.json::<GitHubUser>().await?;

    if !response.status().is_success() {
        let error_message =response 
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        tracing::error!("[auth_token] get user error: {:?}", error_message);
        return Err(AppError::RequestError(
            "non user info in response".to_string(),
        ));
    }

    let user_info: GitHubUser = response
        .json()
        .await
        .map_err(|e| AppError::CustomError(e.to_string() + "Failed to parse user info"))?;

    Ok(Json(serde_json::json!({
        "result": user_info
    })))
}


