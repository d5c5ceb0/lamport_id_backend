use super::proposal_message::*;
use crate::{app::SharedState, common::error::{AppResult,AppError}, server::middlewares::AuthToken, common::consts};
use axum::{debug_handler, extract::{self,State, Query,Path}, Json};

#[debug_handler]
pub async fn create_proposal(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    extract::Json(CreateProposalRequest{data: payload, sig}): extract::Json<CreateProposalRequest>,
) -> AppResult<Json<serde_json::Value>> {
    //TODO check sig

    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();
    let group_id = payload.group_id.clone();
    let title = payload.title;
    let description = payload.description;
    let options = payload.options;
    let start_time = chrono::Utc::now();
    let end_time = payload.end_time;

    //check payload options
    if options.len() < 2 {
        return Err(AppError::InputValidateError("options count must >= 2".into()));
    }
    //check payload title
    if title.is_empty() {
        return Err(AppError::InputValidateError("title must not be empty".into()));
    }
    //check payload description
    if description.is_empty() {
        return Err(AppError::InputValidateError("description must not be empty".into()));
    }
    //check payload description max length
    if description.len() > (consts::PROPOSAL_DESCRIPTION_MAX_LENGTH as usize) {
        return Err(AppError::InputValidateError("description too long".into()));
    }
    //check payload start_time, start time must less than end time and large than now
    if start_time >= end_time && start_time < chrono::Utc::now() {
        return Err(AppError::InputValidateError("start time must less than end time".into()));
    }
    //check payload end_time, end time must large than now
    if end_time < chrono::Utc::now() {
        return Err(AppError::InputValidateError("end time must large than now".into()));
    }

    //check energy
    let energy = state.store.get_user_power(claim.sub.as_str()).await?;
    if energy < (consts::ENERGY_PROPOSAL_VALUE as i64) {
        return Err(AppError::InputValidateError("energy not enough".into()));
    }
    //check payload group_id, group_id must be in database
    state.store.get_group_by_groupid(group_id.as_str()).await?;

    let new_proposal = state.store.create_proposal(title, description, options, claim.sub.clone(), group_id, start_time, end_time).await?;
    tracing::info!("proposal created: {:?}", new_proposal);

    let mut proposal_info = ProposalInfo::from(new_proposal);
    proposal_info.ai_comments = "AI: This proposal has great potential and is in line with community goals.".to_string();

    //award point
    state
        .store
        .award_points(claim.sub.clone(), consts::POINTS_PROPOSAL, consts::POINTS_PROPOSAL_VALUE, "proposal reward")
        .await?;

    //consume energy 
    state
        .store
        .create_energy(claim.sub.clone(), consts::ENERGY_PROPOSAL, consts::ENERGY_PROPOSAL_VALUE)
        .await?;

    Ok(Json(serde_json::json!({
        "result": proposal_info
    })))
}

#[debug_handler]
pub async fn get_proposal_list(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Query(params): Query<GetProposalsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    tracing::info!("group_id: {:?}", id);

    let offset = params.offset;
    let limit = params.limit;

    let proposals = state.store.get_proposals_list_with_votes_by_groupid(id.as_str(), offset, limit).await?;
    tracing::info!("proposals: {:?}", proposals);

    let proposal_infos = proposals.into_iter().map( |(p, v)| {
        let mut info = ProposalInfo::from(p);
        info.ai_comments = "AI: This proposal has great potential and is in line with community goals.".to_string();
        //info.votes = state.store.count_votes_by_proposal_id(info.proposal_id.as_str()).await.unwrap();
        info.votes = v;
        info.earn = v * (consts::POINTS_VOTE_VALUE as u64);
        info.contribution = v * (-consts::ENERGY_VOTE_VALUE as u64);
        info
    }).collect::<Vec<ProposalInfo>>();

    Ok(Json(serde_json::json!({
        "result": {
            "count": proposal_infos.len(),
            "proposals": proposal_infos
        }
    })))
}

#[debug_handler]
pub async fn get_default_proposal_list(
    State(state): State<SharedState>,
    Query(params): Query<GetProposalsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let group = state.store.get_default_group().await?;
    let id = group.group_id;

    tracing::info!("group_id: {:?}", id);

    let offset = params.offset;
    let limit = params.limit;

    let proposals = state.store.get_proposals_list_with_votes_by_groupid(id.as_str(), offset, limit).await?;
    tracing::info!("proposals: {:?}", proposals);

    let proposal_infos = proposals.into_iter().map( |(p, v)| {
        let mut info = ProposalInfo::from(p);
        info.ai_comments = "AI: This proposal has great potential and is in line with community goals.".to_string();
        info.votes = v;
        info.earn = v * (consts::POINTS_VOTE_VALUE as u64);
        info.contribution = v * (-consts::ENERGY_VOTE_VALUE as u64);
        info
    }).collect::<Vec<ProposalInfo>>();

    Ok(Json(serde_json::json!({
        "result": {
            "count": proposal_infos.len(),
            "proposals": proposal_infos
        }
    })))
}

#[debug_handler]
pub async fn get_proposal_detail(
    State(state): State<SharedState>,
    Path(proposal_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {

    let proposal = state.store.get_proposal_by_proposal_id(proposal_id.as_str()).await?;
    let votes = state.store.count_votes_by_proposal_id(proposal.proposal_id.as_str()).await?;

    let mut proposal_info = ProposalInfo::from(proposal.clone());

    proposal_info.ai_comments = "AI: This proposal has great potential and is in line with community goals.".to_string();
    proposal_info.votes = votes;
    proposal_info.earn = votes * (consts::POINTS_VOTE_VALUE as u64);
    proposal_info.contribution = votes * (-consts::ENERGY_VOTE_VALUE as u64);

    
    let votes_for = state.store.count_votes_by_proposal_id_and_choice(proposal.proposal_id.as_str(), "For").await?;
    let votes_against = state.store.count_votes_by_proposal_id_and_choice(proposal.proposal_id.as_str(), "Against").await?;
    let votes_abstain = state.store.count_votes_by_proposal_id_and_choice(proposal.proposal_id.as_str(), "Abstain").await?;

    Ok(Json(serde_json::json!({
        "result": {
            "info": proposal_info,
            "stats": {
                "votes": votes,
                "for:": votes_for,
                "against": votes_against,
                "abstain": votes_abstain,
            }
        }
    })))
}


