use super::referral_message::*;
use crate::{
    app::SharedState, 
    common::{
        error::AppResult, 
        consts
    },
};

pub async fn get_referral_detail(
    state: &SharedState,
    lamport_id: &str,
) -> AppResult<ReferralStatsResponse> {

    let twitter_point = state.store.get_points_by_lamport_id_and_point_type_and_description(lamport_id, consts::POINTS_INVITE, consts::INVITE_TWITTER_CHANNEL).await?;
    let twitter_count = state.store.get_count_points_by_lamport_id_and_point_type_and_description(lamport_id, consts::POINTS_INVITE, consts::INVITE_TWITTER_CHANNEL).await?;

    let twitter = ReferralStats {
        referral_count: twitter_count as i32,
        keys: twitter_point as i32,
    };

    let resp_detail = ReferralStatsDetail {
        twitter,
        telegram: Default::default(),
        others: Default::default(),
    };

    let resp = ReferralStatsResponse {
        total_invite_count: twitter_count as i32,
        total_keys: twitter_point as i32,
        details: resp_detail,
    };
    
    Ok(resp)
        
}
