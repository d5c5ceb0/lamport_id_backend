//use super::referral_message::*;
use super::referral_service::*;
use crate::{
    app::SharedState, 
    common::{
        error::AppResult, 
        consts
    },
    server::{
        middlewares::AuthToken,
        auth::auth_message::*
    },
    database::services::points,
    nostr,
};
use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};

#[debug_handler]
pub async fn get_referral(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let referral_info = get_referral_detail(&state,claim.sub.as_ref()).await?;

    Ok(Json(serde_json::json!({
        "result": referral_info
    })))
        
}
