use super::auth_handler::*;
use crate::app::SharedState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn auth_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/callback", get(callback_handler))
        .route("/token", post(auth_token))
        .route("/csrf_token", get(get_csrf_token))
        .route("/telegram_token", get(get_csrf_token))
        .route("/nonce/:address", get(get_nonce))
        .with_state(state.clone())
}

