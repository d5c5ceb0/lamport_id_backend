use super::contrib_handler::*;
use crate::{
    app::SharedState,
    server::middlewares
};
use axum::{
    middleware,
    routing::get, 
    Router
};

pub fn contrib_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", get(get_contributions))
        .route("/detail", get(get_contributions_detail))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
