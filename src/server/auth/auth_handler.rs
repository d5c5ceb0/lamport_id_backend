use super::auth_message::*;
use super::auth_service::*;
use crate::{
    app::SharedState,
    common::error::AppResult,
    server::middlewares::AuthToken,
};
use axum::{
    debug_handler,
    extract::{Query, State, Path},
    Json,
};

#[debug_handler]
pub async fn callback_handler(
    State(state): State<SharedState>,
    Query(params): Query<OAuthCallbackParams>,
) -> Json<serde_json::Value> {
    tracing::info!("auth params: {:?}", params);

    let resp = auth_callback_handler(&state, &params).await;

    Json(serde_json::json!({
        "result": resp
    }))
}

#[debug_handler]
pub async fn get_csrf_token(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {

    let token= auth_get_csrf_token(&state).await?;

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

    let token= auth_get_nonce(&state, address).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "nonce": token
        }
    })))
}

#[debug_handler]
pub async fn get_tg_token(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let token= auth_get_tg_token(&state, claim.sub).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "tg_token": token
        }
    })))
}


