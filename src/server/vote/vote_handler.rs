use super::vote_message::*;
use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::Path, extract::State, extract::Query, extract::Json as EJson, Json};
use crate::database::entities::vote;
use sea_orm::*;


//impl axum create vote handler
#[debug_handler]
pub async fn create_vote(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    EJson(mut vote_info): EJson<VoteInfo>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    vote_info.voter_id = Some(claim.sub);

    let active_vote = vote_info.into();

    let created_vote = state.store.create_vote(active_vote).await?;

    Ok(Json(serde_json::json!({
        "result": VoteInfo::from(created_vote)
        
    })))
}

//impl axum get votes by proposal_id handler
#[debug_handler]
pub async fn get_votes_by_proposal_id(
    State(state): State<SharedState>,
    Path(proposal_id): Path<String>,
    Query(params): Query<GetVotesRequest>,
) -> AppResult<Json<serde_json::Value>> {

    let offset = params.offset;
    let limit = params.limit;

    //TODO page list
    let votes = state.store.get_votes_by_proposal_id(proposal_id.as_str(), offset, limit).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "count": votes.len(),
            "votes": votes.into_iter().map(VoteInfo::from).collect::<Vec<VoteInfo>>()
        }
    })))
}

//impl axum get votes by voter_id handler
#[debug_handler]
pub async fn get_votes_by_voter_id(
    State(state): State<SharedState>,
    Path(voter_id): Path<String>,
    Query(params): Query<GetVotesRequest>,
) -> AppResult<Json<serde_json::Value>> {

    let offset = params.offset;
    let limit = params.limit;

    let votes = state.store.get_votes_by_voter_id(voter_id.as_str(), offset, limit).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "count": votes.len(),
            "votes": votes.into_iter().map(VoteInfo::from).collect::<Vec<VoteInfo>>()
        }
    })))
}

//impl axum count votes by proposal_id and choice handler
#[debug_handler]
pub async fn count_votes_by_proposal_id_and_choice(
    State(state): State<SharedState>,
    Query(GetChoiceCountRequest{proposal_id,choice}): Query<GetChoiceCountRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let count = state.store.count_votes_by_proposal_id_and_choice(proposal_id.as_str(), choice.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "count": count
        }
    })))
}
