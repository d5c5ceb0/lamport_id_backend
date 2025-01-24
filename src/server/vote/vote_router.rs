use super::vote_handler::*;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::{get,post}, Router};

pub fn vote_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", post(create_vote))
        .route("/voter_votes/:voter_id", get(get_votes_by_voter_id))
        .route("/:proposal_id", get(get_proposal_vote_by_voter_id))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
        .route("/choice_count", get(count_votes_by_proposal_id_and_choice))
        .route("/proposal_votes/:proposal_id", get(get_votes_by_proposal_id))
}

