use super::proposal_message::*;
use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::{self,State, Query,Path}, Json};
use serde::{Deserialize, Serialize};
use crate::database::entities::proposal;


#[debug_handler]
pub async fn create_proposal(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    extract::Json(payload): extract::Json<CreateProposalRequest>,
) -> AppResult<Json<serde_json::Value>> {

    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let title = payload.title;
    let description = payload.description;
    let options = payload.options;
    let start_time = payload.start_time;
    let end_time = payload.end_time;

    //get group id by lamport_id
    let group = state.store.get_group_by_creator(claim.sub.as_str()).await?;

    let new_proposal = state.store.create_proposal(title, description, options, group.created_by, start_time, end_time).await?;
    tracing::info!("proposal created: {:?}", new_proposal);

    let proposal_info = ProposalInfo::from(new_proposal);

    Ok(Json(serde_json::json!({
        "result": proposal_info
    })))
}

#[debug_handler]
pub async fn get_proposal_list_by_creator(
    State(state): State<SharedState>,
    Path(creator): Path<String>,
    Query(params): Query<GetProposalsRequest>,
) -> AppResult<Json<serde_json::Value>> {

    //let creator = params.creator;
    let offset = params.offset;
    let limit = params.limit;

    let proposals = state.store.get_proposals_list_by_creator(creator, offset, limit).await?;
    tracing::info!("proposals: {:?}", proposals);

    let proposal_infos = proposals.into_iter().map(ProposalInfo::from).collect::<Vec<ProposalInfo>>();

    Ok(Json(serde_json::json!({
        "result": {
            "count": proposal_infos.len(),
            "proposals": proposal_infos
        }
    })))
}

