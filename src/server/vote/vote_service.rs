//use super::vote_message::*;
use crate::{
    app::SharedState,
    common::error::AppResult,
    helpers::redis_cache::*,
};

const VOTE_COUNT_KEY: &str = "vote:count";
const VOTE_VOTER_COUNT_KEY: &str = "vote:voter:count";

pub async fn count_votes_by_group_id(state: &SharedState, group_id: &str) -> AppResult<i64> {
    let cache = RedisClient::from(state.redis.clone());

    let key = format!("{}:{}", VOTE_COUNT_KEY, group_id);

    if let Ok(data) = cache.get_data(&key).await {
        tracing::info!("get data from cache:{:?}", data);
        return Ok(data);
    }

    let vote_count = state.store.count_votes_by_group_id(group_id).await?;

    cache.set_data(&key, &vote_count).await?;

    Ok(vote_count)


}

pub async fn count_voters_by_group_id(state: &SharedState, group_id: &str) -> AppResult<i64> {
    let cache = RedisClient::from(state.redis.clone());

    let key = format!("{}:{}", VOTE_VOTER_COUNT_KEY, group_id);

    if let Ok(data) = cache.get_data(&key).await {
        tracing::info!("get data from cache:{:?}", data);
        return Ok(data);
    }

    let voter_count = state.store.count_voters_by_group_id(group_id).await?;

    cache.set_data(&key, &voter_count).await?;

    Ok(voter_count)
}

