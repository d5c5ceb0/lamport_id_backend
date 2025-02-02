use crate::common::consts;
use crate::{
    common::error::AppResult,
    helpers::redis_cache::*,
    app::SharedState,
};

const PROPOSAL_COUNT_KEY: &str = "proposal:count";

//get proposal status: little than start_time, between start_time and end_time, greater than end_time
pub fn get_proposal_status(start_time: chrono::DateTime<chrono::Utc>, end_time: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    if now < start_time {
        consts::PROPOSAL_STATUS_PENDING.to_string()
    } else if now >= start_time && now <= end_time {
        consts::PROPOSAL_STATUS_ACTIVE.to_string()
    } else {
        consts::PROPOSAL_STATUS_PASSED.to_string()
    }
}


pub async fn count_proposals_by_groupid(state: &SharedState, group_id: &str) -> AppResult<i64> {
    let cache = RedisClient::from(state.redis.clone());

    let key = format!("{}:{}", PROPOSAL_COUNT_KEY, group_id);

    if let Ok(data) = cache.get_data(&key).await {
        tracing::info!("get data from cache:{:?}", data);
        return Ok(data);
    }
    let proposal_count = state.store.count_proposals_by_groupid(group_id).await?;

    cache.set_data(&key, &proposal_count).await?;

    Ok(proposal_count as i64)
}
