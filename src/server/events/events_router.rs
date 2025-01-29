use super::events_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::get, Router};

pub fn events_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/:lamportid", get(get_events))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
