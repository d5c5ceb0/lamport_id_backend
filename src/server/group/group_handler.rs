use super::group_message::*;
use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::Json as EJson, extract::State, extract::Query, Json};
use serde::Deserialize;
use crate::database::entities::group;
use serde::Serialize;
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
