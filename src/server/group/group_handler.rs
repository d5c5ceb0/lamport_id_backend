use super::group_message::*;
use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::Json as EJson, extract::State, extract::Query, Json};
use std::convert::Into;


#[debug_handler]
pub async fn create_group(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    EJson(CreateGroupRequest {
        name,
        logo,
        description,
        website,
        twitter,
    }): EJson<CreateGroupRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let new_group = state.store.create_group(name, logo, description, website, twitter, claim.sub).await?;
    let group_info = GroupInfo::from(new_group);

    Ok(Json(serde_json::json!({
        "result": group_info
    })))
}


#[debug_handler]
pub async fn get_group_list(
    State(state): State<SharedState>,
    //AuthToken(user): AuthToken,
    Query(GetGroupListRequest { offset, limit }): Query<GetGroupListRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let groups = state.store.get_group_list(offset, limit).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "count": groups.len(),
            "groups":groups.into_iter().map(GroupInfo::from).collect::<Vec<GroupInfo>>()
        }
    })))
}

#[debug_handler]
pub async fn get_group_info(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {
    let group = state.store.get_default_group().await?;
    let group_info = GroupInfo::from(group.clone());

    let proposals = state.store.count_proposals_by_groupid(group.group_id.as_str()).await?;

    let votes = state.store.count_votes_by_group_id(group.group_id.as_str()).await?;

    let members = state.store.count_voters_by_group_id(group.group_id.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "info": group_info,
            "stats": {
                "ai_score": 5,
                "ai_rating": 211,
                "particaipation_proposals": 105,
                "recommended_proposals": 105,
                "activity_contribution": 250,
                "daily_average_msg": 50,
                "members": members,
                "proposals": proposals,
                "votes": votes,
            }
        }
    })))
}
