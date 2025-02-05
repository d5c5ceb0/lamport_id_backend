use super::group_message::*;
use crate::{
    common::error::AppResult,
    helpers::redis_cache::*,
    app::SharedState,
    server::{
        vote::vote_service::{
            //count_votes_by_group_id,
            //count_voters_by_group_id,
        },
        //proposal::proposal_service::count_proposals_by_groupid,
    }
};

const GROUP_LIST_KEY : &str = "group:list";
const DEFAULT_GROUP_KEY : &str = "group:default";
const GROUP_INFO_KEY : &str = "group:info";


pub async fn create_group(
    state: &SharedState,
    name: String,
    logo: String,
    description: Option<String>,
    website: String,
    twitter: String,
    user_id: String,
) -> AppResult<GroupInfo> {
    let cache = RedisClient::from(state.redis.clone());

    //check name TODO

    let new_group = state.store.create_group(name, logo, description, website, twitter, user_id).await?;

    //invlidate cache
    cache.invalidate_cache(format!("{}:*", GROUP_LIST_KEY).as_str()).await?;

    Ok(GroupInfo::from(new_group))
}

pub async fn get_group_list(state: &SharedState, offset: i64, limit: i64) -> AppResult<Vec<GroupInfo>> {
    let cache = RedisClient::from(state.redis.clone());

    let key = format!("{}:{}:{}", GROUP_LIST_KEY, offset, limit);

    if let Ok(data) = cache.get_data(&key).await {
        tracing::info!("get data from cache:{:?}", data);
        return Ok(data);
    }

    let groups = state.store.get_group_list(offset, limit).await?;
    let group_infos = groups.into_iter().map(GroupInfo::from).collect::<Vec<GroupInfo>>();

    cache.set_data(&key, &group_infos).await?;

    Ok(group_infos)
}


pub async fn get_default_group(state: &SharedState) -> AppResult<GroupInfo> {
    let cache = RedisClient::from(state.redis.clone());

    let key = DEFAULT_GROUP_KEY;

    if let Ok(data) = cache.get_data(key).await {
        tracing::info!("get data from cache:{:?}", data);
        return Ok(data);
    }

    let group = state.store.get_default_group().await?;
    let group_info = GroupInfo::from(group);

    cache.set_data(key, &group_info).await?;

    Ok(group_info)
}

pub async fn get_group_info(state: &SharedState, group_id: &str) -> AppResult<GroupInfo> {
    let cache = RedisClient::from(state.redis.clone());

    let key = format!("{}:{}", GROUP_INFO_KEY, group_id);

    if let Ok(data) = cache.get_data(&key).await {
        tracing::info!("get data from cache:{:?}", data);
        return Ok(data);
    }

    let group = state.store.get_group_by_groupid(group_id).await?;
    let group_info = GroupInfo::from(group);

    cache.set_data(&key, &group_info).await?;

    Ok(group_info)
}

pub async fn get_group_stats(state: &SharedState, group_id: &str) -> AppResult<GroupStats> {
    //let proposals = count_proposals_by_groupid(state, group_id).await?;
    //let votes = count_votes_by_group_id(state, group_id).await?;
    //let members = count_voters_by_group_id(state, group_id).await?;

    let proposals = state.store.count_proposals_by_groupid(group_id).await? as i64;
    let votes = state.store.count_votes_by_group_id(group_id).await?;
    let members = state.store.count_voters_by_group_id(group_id).await?;

    let group_state = GroupStats {
        ai_score: 5,
        ai_rating: 211,
        particaipation_proposals: 105,
        recommended_proposals: 105,
        activity_contribution: 250,
        daily_average_msg: 50,
        members,
        proposals,
        votes,
    };

    Ok(group_state)
}

