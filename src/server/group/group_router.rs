use super::group_handler::{create_group, get_group_list, get_group_info};
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get, post}, Router};

pub fn group_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/create", post(create_group))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/", get(get_group_info))
        .route("/list", get(get_group_list))
}
