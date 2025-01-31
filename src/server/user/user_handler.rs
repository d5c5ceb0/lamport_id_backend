use super::user_message::*;
use crate::{app::SharedState, common::error::{AppResult, AppError}, server::middlewares::AuthToken};
use axum::{debug_handler, extract::State, Json};
use crate::server::auth::{auth_message::*};
use reqwest::Client;
use crate::common::consts;
use serde::{Deserialize, Serialize};
use crate::database::entities::twitter_binding;
use crate::nostr;

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


    let created_binding= match state.store.binding_twitter(
        claim.sub.clone(),
        user_info.data.id.clone(),
        user_info.data.name.clone(),
        user_info.data.username.clone(),
        user_info.data.profile_image_url.clone(),
        token.access_token.clone(),
        token.refresh_token.clone(),
        token.token_type.clone(),
        token.scope.clone(),
    ).await {
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

impl From<twitter_binding::Model> for BindingTwitterResponse {
    fn from(binding: twitter_binding::Model) -> Self {
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
