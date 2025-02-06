use super::user_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get,post}, Router};

pub fn user_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/user/info", get(get_user_info))
        .route("/user/stats", get(get_user_stats))
        .route("/user/bindings", post(binding_twitter).get(get_user_bindings))
        .route("/user/binding/telegram", post(binding_telegram))
        .route("/user/binding/discord", post(binding_discord))
        .route("/user/binding/github", post(binding_github))
        .route("/users/info", get(get_user_info))
        .route("/users/stats", get(get_user_stats))
        .route("/users/verify/:address", post(verify_user))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/users/login", post(login))
        .route("/users", post(register))
        .route("/users/:username", get(check_username))
        .route("/user/count", get(get_user_count))
}
