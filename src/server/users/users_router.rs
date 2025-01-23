use super::users_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use crate::server::user::user_handler::*;
use axum::{middleware, routing::{get,post}, Router};

pub fn users_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/info", get(get_user_info))
        .route("/stats", get(get_user_stats))
        .route("/verify/:address", post(verify_user))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/login", post(login))
        .route("/", post(register))
        .route("/:username", get(check_username))
}

