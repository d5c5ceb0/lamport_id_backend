use super::referral_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::get, Router};

pub fn referral_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", get(get_referral))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}

