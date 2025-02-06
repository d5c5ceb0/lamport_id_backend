use super::user_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get,post}, Router};

pub fn user_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/info", get(get_user_info))
        .route("/count", get(get_user_count))
        .route("/stats", get(get_user_stats))
        .route("/bindings", post(binding_account).get(get_user_bindings))
        .route("/binding/telegram", post(binding_telegram))
        .route("/binding/discord", post(binding_discord))
        .route("/binding/github", post(binding_github))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
