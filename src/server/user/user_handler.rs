use super::user_message::*;
use super::user_service::*;
use crate::{
    app::SharedState, 
    common::error::{AppResult, AppError}, 
    server::{
        middlewares::AuthToken,
        auth::auth_message::*,
    },
};
use axum::{debug_handler, extract::State, extract::Path, extract::Json as EJson,Json};

#[debug_handler]
pub async fn get_user_info(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let resp = user_get_user_info(&state, claim.sub.as_str()).await?;
    tracing::info!("get user info: {:?}", resp);

    Ok(Json(serde_json::json!({
    "result": resp
    })))
}

#[debug_handler]
pub async fn get_user_count(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {
    let count = user_get_user_count(&state).await?;

    tracing::info!("get user count: {:?}", count);

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

    let resp = user_get_user_stats(&state, claim.sub.as_str()).await?;

    Ok(Json(serde_json::json!({
    "result": resp
    })))
}

//check username
#[debug_handler]
pub async fn check_username(
    State(state): State<SharedState>,
    Path(username): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    tracing::info!("check username: {:?}", username);
    match user_check_username(&state, username.as_str()).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "result": "OK"
        })),
        ),
        Err(e) => Err(e),
    }
}

#[debug_handler]
pub async fn verify_user(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Path(address): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();
    
    user_verify_user(&state, claim.sub.as_str(), address.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "status": "success",
            "address": address
        }
    })))
}

#[debug_handler]
pub async fn login(
    State(state): State<SharedState>,
    EJson(req): EJson<LoginRequest>,
) -> AppResult<Json<serde_json::Value>> {

    let (token, user) = user_login(&state, req).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": user
        }
    })))
}

#[debug_handler]
pub async fn register(
    State(state): State<SharedState>,
    EJson(req): EJson<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (token, user) = user_register(&state, req).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": user
        }
    })))
}

#[debug_handler]
pub async fn binding_twitter(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    tracing::info!("[auth_token] get params: {:?}", params);

    params.validate_items()?;

    let resp = user_binding_twitter(&state, claim.sub, params.clone()).await?;


    Ok(Json(serde_json::json!({
        "result": {
            "twitter_info": resp
        }
    })))
}

#[debug_handler]
pub async fn get_user_bindings(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    match user_get_user_bindings(&state, claim.sub.as_str()).await {
        Ok(bindings) => {
            tracing::info!("get user bindings: {:?}", bindings);
            Ok(Json(serde_json::json!({
                "result": {
                    "twitter_info": bindings.0,
                    "telegram_info": bindings.1,
                    "discord_info": bindings.2,
                    "github_info": bindings.3,
                }
            })),
        )
        }
        Err(_e) => Ok(Json(serde_json::json!({
            "result": {
                "twitter_info": {},
                "telegram_info": {},
                "discord_info": {},
                "github_info": {},
            }
        }))),
    }
}

pub async fn binding_telegram(
    State(state): State<SharedState>,
    Json(params): Json<TelegramParams>,
) -> AppResult<Json<serde_json::Value>> {
    let binding = user_binding_telegram(&state, &params).await?;

    Ok(Json(serde_json::json!({
        "result": binding
    })))
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

    let user_info = user_binding_discord(&state, claim.sub.as_str(), &params).await?;

    Ok(Json(serde_json::json!({
        "result": user_info
    })))
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

    let user_info = user_binding_github(&state, claim.sub.as_str(), &params).await?;

    Ok(Json(serde_json::json!({
        "result": user_info
    })))
}



