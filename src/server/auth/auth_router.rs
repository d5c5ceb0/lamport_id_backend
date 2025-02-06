use super::auth_handler::*;
use crate::{
    app::SharedState,
    server::middlewares
};
use axum::{middleware, routing::get, Router};

pub fn auth_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/telegram_token", get(get_tg_token))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/callback", get(callback_handler))
        .route("/csrf_token", get(get_csrf_token))
        .route("/nonce/:address", get(get_nonce))
}

