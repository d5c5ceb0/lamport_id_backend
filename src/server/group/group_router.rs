use super::group_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get, post}, Router};

pub fn group_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", post(create_group))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/", get(get_group_info))
        .route("/list", get(get_group_list))
        .route("/:group_id", get(get_group_info_by_groupid))
}
