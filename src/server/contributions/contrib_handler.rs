use super::contrib_message::*;
use super::contrib_service::*;
use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::State, extract::Query, Json};

#[debug_handler]
pub async fn get_contributions(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let twitter = fetch_twitter_contributions(&state, claim.sub.as_str()).await?;
    

    Ok(Json(serde_json::json!({
    "result": {
        "twitter": twitter,
        "telegram": 0,
        "discord": 0,
    }
    })))
}

pub async fn get_contributions_detail(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Query(GetContributionsDetailRequest { media , offset, limit }): Query<GetContributionsDetailRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();


    let x_list = fetch_twitter_contributions_detail(
        &state,
        claim.sub.as_str(),
        offset,
        limit,
    ).await?;


    Ok(Json(serde_json::json!({
        "result": x_list,
    })))
}
