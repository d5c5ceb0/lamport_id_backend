use super::proposal_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get,post}, Router};

pub fn proposal_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", post(create_proposal))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth_middleware,
        ))
        .route("/list/:group_id", get(get_proposal_list))
        .route("/list", get(get_default_proposal_list))
        .route("/detail/:proposal_id", get(get_proposal_detail))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::eip191_middleware,
        ))
}
