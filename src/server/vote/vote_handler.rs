use super::vote_message::*;
use crate::{
    app::SharedState, 
    common::error::{AppResult, AppError}, 
    server::{middlewares::AuthToken, proposal::proposal_service::get_proposal_status, events::events_message::Event }, 
    common::consts,
    helpers::eip191::verify_signature,
};
use axum::{debug_handler, extract::Path, extract::State, extract::Query, extract::Json as EJson, Json};


//impl axum create vote handler
#[debug_handler]
pub async fn create_vote(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    EJson(CreateVoteRequest{data,sig}): EJson<CreateVoteRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();  //TODO address in jwt

    if cfg!(not(debug_assertions)) {
        let unverified_data = UnVerifyVoteInfo {
            proposal_id: data.proposal_id.clone(),
            choice: data.choice.clone(),
            channel: data.channel.clone(),
        };
        //get user address by lamport id
        let user = state.store.get_user_by_uid(claim.sub.as_str()).await?;

        let verified= verify_signature(&unverified_data, &sig, &user.address)?;
        if !verified {
            return Err(AppError::InvalidSignature);
        }
    }

    let mut vote_info = data.clone();

    //check energy
    let energy = state.store.get_user_power(claim.sub.as_str()).await?;
    if energy < (consts::ENERGY_PROPOSAL_VALUE as i64) {
        return Err(AppError::InputValidateError("energy not enough".into()));
    }
    //check vote_info.proposal_id, proposal_id must be in database, porposal_id must be active
    let proposal = state.store.get_proposal_by_proposal_id(vote_info.proposal_id.as_str()).await?;
    if get_proposal_status(proposal.start_time.into(), proposal.end_time.into()) != consts::PROPOSAL_STATUS_ACTIVE {
        return Err(AppError::InputValidateError("proposal not active".into()));
    }

    //checkout vote_info.choice, choice must be in proposal options
    if !proposal.options.contains(&vote_info.choice) {
        return Err(AppError::InputValidateError("choice not in proposal options".into()));
    }

    //checkout vote_info.voter_id, voter_id must not be voted before
    if state.store.is_voted_by_voter_id(claim.sub.as_str(), vote_info.proposal_id.as_str()).await? {
        return Err(AppError::InputValidateError("voter has voted before".into()));
    }

    vote_info.voter_id = Some(claim.sub.clone());

    let active_vote = vote_info.into();

    let created_vote = state.store.create_vote(active_vote).await?;

    //award point
    state
        .store
        .award_points(claim.sub.clone(), consts::POINTS_VOTE, consts::POINTS_VOTE_VALUE, "vote reward")
        .await?;

    //consume energy 
    state
        .store
        .create_energy(claim.sub.clone(), consts::ENERGY_VOTE, consts::ENERGY_VOTE_VALUE)
        .await?;

    if state.store.count_votes_by_voter_id(claim.sub.as_str()).await? == 1 {
        let queue = state.queue.clone();

        let e = Event {
            event_id: uuid::Uuid::new_v4().to_string(),
            lamport_id: claim.sub.clone(),
            event_type: consts::EVENT_TYPE_VOTE.to_string(),
            content: "First vote cast".to_string(),
            created_at: chrono::Utc::now(),
        };
        queue.add_queue_req_ex(consts::EVENT_TOPIC, e).await?;

    }

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
    tracing::info!("voter_id: {:?}", voter_id);

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

//get vote info of voter_id and proposal_id
#[debug_handler]
pub async fn get_proposal_vote_by_voter_id(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Path(proposal_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    match state.store.get_proposal_vote_by_voter_id(claim.sub.as_str(), proposal_id.as_str()).await {
        Ok(v) => Ok(Json(serde_json::json!({
            "result": VoteInfo::from(v)
        }))),
        Err(AppError::CustomError(_)) => Ok(Json(serde_json::json!({
                "result": ""
            }))),
        Err(e) => {
            Err(e)
        }
    }
}
