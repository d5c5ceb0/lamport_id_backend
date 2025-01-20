use super::proposal_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get,post}, Router};

pub fn proposal_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/create", post(create_proposal))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/list", get(get_proposal_list_by_creator))
}
