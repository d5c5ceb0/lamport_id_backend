use super::{
    auth::auth_router,
    health::health_router,
    user::user_router,
    webset::index_router,
    group::group_router,
    proposal::proposal_router,
    vote::vote_router,
    users::users_router,
    events::events_router,
    contributions::contrib_router,
    referrals::referral_router,
};
use crate::{app::SharedState, server::middlewares};
use axum::{error_handling::HandleErrorLayer, http::Method, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn app_router(state: SharedState) -> Router {
    let user_router = user_router(state.clone());
    let users_router = users_router(state.clone());
    let auth_router = auth_router(state.clone());
    let index_router = index_router();
    let health_router = health_router();
    let group_router = group_router(state.clone());
    let proposal_router = proposal_router(state.clone());
    let vote_router = vote_router(state.clone());
    let events_router = events_router(state.clone());
    let contrib_router = contrib_router(state.clone());
    let referral_router = referral_router(state.clone());

    Router::new()
        .nest("/", index_router)
        .nest("/api/v1/health", health_router)
        .nest("/api/v1/auth", auth_router)
        .nest("/api/v1/user", user_router)
        .nest("/api/v1/users", users_router)
        .nest("/api/v1/group", group_router)
        .nest("/api/v1/proposal", proposal_router)
        .nest("/api/v1/vote", vote_router)
        .nest("/api/v1/events", events_router)
        .nest("/api/v1/contributions", contrib_router)
        .nest("/api/v1/referral", referral_router)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(middlewares::handle_error))
                .timeout(Duration::from_secs(600))
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(
                            |request: &axum::http::Request<axum::body::Body>,
                             _: &tracing::span::Span| {
                                let method = request.method();
                                let uri = request.uri();
                                tracing::info!("Received request: {} {}", method, uri);
                            },
                        )
                        .on_response(
                            |response: &axum::http::Response<axum::body::Body>,
                             latency: Duration,
                             _: &tracing::span::Span| {
                                let status = response.status();
                                tracing::info!(
                                    "Sending response: {} with status: {}",
                                    latency.as_secs_f64(),
                                    status
                                );
                            },
                        ),
                ),
        )
        .with_state(state.clone())
}
