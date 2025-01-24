use super::auth_message::*;
use super::auth_service::*;
use crate::{
    app::SharedState,
    common::error::{AppError, AppResult},
    server::user::*,
    common::consts,
};
use axum::{
    debug_handler,
    extract::{Query, State, Path},
    Json,
};
//use oauth2::RedirectUrl;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeTokenRespose {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
}


#[debug_handler]
pub async fn auth_token(
    State(state): State<SharedState>,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    tracing::info!("[auth_token] get params: {:?}", params);

    params.validate_items()?;

    //let client = state
    //    .oauth
    //    .clone()
    //    .set_redirect_uri(RedirectUrl::new(params.clone().redirect_uri.unwrap().clone())?);
    //    //.set_redirect_uri(RedirectUrl::new(state.config.auth.redirect_url.clone())?);

    let csrf_state = params.clone()
        .state
        .ok_or(AppError::InputValidateError("Invild state".into()))?;

    let redis_client = RedisClient::from(state.redis.clone());
    if let Ok(token) = redis_client.get_csrf_token(csrf_state.as_str()).await {
        tracing::info!("got csrf token: {:?} by key: {:?}", token, csrf_state);
    } else {
        tracing::error!("got csrf token err: wrong state:{:?} ", csrf_state);
        //return Err(AppError::InputValidateError(
        //    "csrf token verification error".into(),
        //));
    }

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

    let access_token = token.access_token;

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

    let created_user = if state
        .store
        .is_user_exists_by_username(user_info.data.username.as_ref())
        .await?
    {
        state
            .store
            .get_user_by_username(user_info.data.username.as_ref())
            .await?
    } else {
        let user: User = User::from(user_info.clone());
        tracing::info!("[auth_token] create user: {:?}", user);

        //points
        let user = match params.invited_by {
            Some(invited) => user.add_invited_by(invited.as_str()),
            None => user,
        };

        let created_user = match state.store.create_user(user.into()).await {
            Ok(u) => u,
            Err(AppError::UserExisted(_)) => {
                tracing::info!("user has already existed, log in");
                state
                    .store
                    .get_user_by_username(user_info.data.username.as_ref())
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
                .award_points(inviter.lamport_id.clone(), consts::POINTS_INVITE, consts::POINTS_INVITE_VALUE, "invite reward")
                .await?;

            //consume energy 
            state
                .store
                .create_energy(inviter.lamport_id, consts::ENERGY_INVITE, consts::ENERGY_INVITE_VALUE)
                .await?;

        }


        tracing::info!("[auth_token] database  user info: {:?}", created_user);
        created_user
    };

    let secret = state.jwt_handler.clone();
    let token: String =
        secret.create_token(&created_user.lamport_id, &created_user.name, &created_user.user_name);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    //redis_client
    //    .del_csrf_token(csrf_state.as_str())
    //    .await
    //    .unwrap();

    Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": UserResponse::from(created_user)
        }
    })))
}

#[debug_handler]
pub async fn callback_handler(
    State(state): State<SharedState>,
    Query(params): Query<OAuthCallbackParams>,
    //Query(params): Query<HashMap<String, String>>
) -> Json<serde_json::Value> {
    tracing::info!("auth params: {:?}", params);

    Json(serde_json::json!({
        "result": {
            "code": params.code,
            "state": "authorization_code",
            "redirect_uri": state.config.auth.redirect_url.clone()
        }
    }))
}

#[debug_handler]
pub async fn get_csrf_token(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {
    let redis_client = RedisClient::from(state.redis.clone());
    let token = redis_client.cache_csrf_token().await.unwrap();
    tracing::info!("gen csrf token: {:?}", token);

    Ok(Json(serde_json::json!({
        "result": {
            "csrf_token": token
        }
    })))
}

#[debug_handler]
pub async fn get_nonce(
    State(state): State<SharedState>,
    Path(address): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let redis_client = RedisClient::from(state.redis.clone());
    let token = redis_client.cache_nonce(address.as_str()).await.unwrap();
    tracing::info!("gen nonce: {}-{:?}", address, token);

    Ok(Json(serde_json::json!({
        "result": {
            "nonce": token
        }
    })))
}

