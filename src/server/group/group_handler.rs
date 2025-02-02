use super::group_message::*;
use super::group_service;
use crate::{
    app::SharedState, 
    common::error::AppResult, 
    server::middlewares::AuthToken
};
use axum::{
    debug_handler, 
    extract::{
        Json as EJson,
        State,
        Query,
        Path
    }, 
    Json
};
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

    let group_info = group_service::create_group(
        &state,
        name,
        logo,
        description,
        website,
        twitter,
        claim.sub,
    ).await?;

    Ok(Json(serde_json::json!({
        "result": group_info
    })))
}


#[debug_handler]
pub async fn get_group_list(
    State(state): State<SharedState>,
    Query(GetGroupListRequest { offset, limit }): Query<GetGroupListRequest>,
) -> AppResult<Json<serde_json::Value>> {

    let groups = group_service::get_group_list(&state, offset, limit).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "count": groups.len(),
            "groups":groups
        }
    })))
}

#[debug_handler]
pub async fn get_group_info(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {

    let group_info = group_service::get_default_group(&state).await?;
    let group_state = group_service::get_group_stats(&state, group_info.group_id.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "info": group_info,
            "stats": group_state
        }
    })))
}

#[allow(unused)]
#[debug_handler]
pub async fn get_group_info_by_groupid(
    State(state): State<SharedState>,
    Path(group_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {

    let group_info = group_service::get_group_info(&state,group_id.as_str()).await?;
    let group_state = group_service::get_group_stats(&state, group_id.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "info": group_info,
            "stats": group_state
        }
    })))
}
