use super::auth_message::*;
use crate::{
    app::SharedState,
    common::error::AppResult,
    helpers::redis_cache::*,
};
use serde::{Deserialize, Serialize};

pub async fn auth_callback_handler(
    state: &SharedState,
    params: &OAuthCallbackParams,
) -> AuthResponse {
    tracing::info!("auth params: {:?}", params);

    AuthResponse::new(
        params.code.clone(),
        "authorization_code".to_string(),
        state.config.auth.redirect_url.clone(),
    )
}

pub async fn auth_get_csrf_token(
    state: &SharedState,
) -> AppResult<String> {
    let redis_client = RedisClient::from(state.redis.clone());
    let token = redis_client.cache_csrf_token().await?;
    tracing::info!("gen csrf token: {:?}", token);

    Ok(token)
}

pub async fn auth_get_nonce(
    state: &SharedState,
    address: String,
) -> AppResult<String> {
    let redis_client = RedisClient::from(state.redis.clone());
    let token = redis_client.cache_nonce(address.as_str()).await?;
    tracing::info!("gen nonce: {}-{:?}", address, token);

    Ok(token)
}

