use super::contrib_message::*;
use crate::{app::SharedState, common::{consts, error::{AppError, AppResult}}};
use crate::helpers::redis_cache::*;


pub async fn fetch_twitter_contributions(
    state: &SharedState,
    lamport_id: &str,
) -> AppResult<TwitterContribution> {

    //get user by lamport_id
    let user_x_binding = state.store.get_twitter_binding_by_user_id(lamport_id).await?;

     let response = if user_x_binding.updated_at.date_naive() != chrono::Utc::now().date_naive() ||
         (user_x_binding.updated_at.timestamp() - chrono::Utc::now().timestamp() )> 1  || true
     {
         //let response = match reqwest::get(&format!("{}/api/user/interactions/{}", consts::TWITTER_SERVICE_URL, user_x_binding.x_id)).await {
         let response = match reqwest::get(&format!("{}/api/user/interactions/{}", consts::TWITTER_SERVICE_URL, "993673319512653824")).await {
             Ok(response) => response,
             Err(e) => {
                 tracing::error!("fetch_twitter_contributions error: {}", e);
                 return Ok(TwitterContribution::default())
             },
         };

         if !response.status().is_success() {
             let error_message = response 
                 .text()
                 .await
                 .unwrap_or_else(|_| "Failed to read error response".to_string());
             tracing::error!("Failed to exchange code: {}", error_message);

                 return Ok(TwitterContribution::default());
         }

         let twitter_response: TwitterResponse = match response.json().await.map_err(|e| AppError::CustomError(e.to_string())) {
             Ok(twitter_response) => twitter_response,
             Err(e) => {
                 tracing::error!("fetch_twitter_contributions error: {}", e);
                 return Ok(TwitterContribution::default())
             },
         };

         tracing::info!("-----{:?}", twitter_response);

         state.store.update_twitter_binding_by_lamport_id(
             lamport_id,
             twitter_response.interaction_summary.retweet,
             0,
             twitter_response.interaction_summary.reply,
             twitter_response.interaction_summary.quote,
         ).await?;

         state.store.get_twitter_binding_by_user_id(lamport_id).await?
     } else {
         tracing::info!("----fetch_twitter_contributions from db");
         user_x_binding
     };
     tracing::info!("----fetch_twitter_contributions from db:{:?}",response);

     Ok(response.into())
}

pub async fn fetch_twitter_contributions_detail(
    state: &SharedState,
    lamport_id: &str,
    offset: i64,
    limit: i64,
) -> AppResult<Vec<TwitterDetail>> {

    //get user by lamport_id
    let user_x_binding = state.store.get_twitter_binding_by_user_id(lamport_id).await?;

    let cache = RedisClient::from(state.redis.clone());

    let key = format!("{}:{}:{}:{}:{}", "twitter_binding", "detail", lamport_id, offset, limit);

    //if let Ok(data) = cache.get_data(&key).await {
    //    tracing::info!("get data from cache:{:?}", data);
    //    return Ok(data);
    //}

    //interaction/:media_account?username=xxx&page=1&pre_page=10
    //let response = match reqwest::get(&format!("{}/api/interaction/{}?username={}", consts::TWITTER_SERVICE_URL, "hetu_protocol", user_x_binding.user_name)).await {
    let response = match reqwest::get(&format!("{}/api/interaction/{}?username={}", consts::TWITTER_SERVICE_URL, "hetu_protocol", "Sky201805")).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("fetch_twitter_contributions_detail error: {}", e);
            return Ok(vec![])
        },
    };

    if !response.status().is_success() {
        let error_message = response 
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        tracing::error!("Failed to exchange code: {}", error_message);

        return Ok(vec![]);
    }

    let twitter_response: TwitterInteractions = match response.json().await.map_err(|e| AppError::CustomError(e.to_string())) {
        Ok(twitter_response) => twitter_response,
        Err(e) => {
            tracing::error!("fetch_twitter_contributions_detail error: {:?}", e);
            return Ok(vec![])
        },
    };

    tracing::info!("-----{:?}", twitter_response);

    //for twitter_response do convert to twitter_detail
    let twitter_detail = twitter_response.interactions.into_iter().map(|interaction| {
        TwitterDetail::from(interaction) 
    }).collect::<Vec<TwitterDetail>>();

    cache.set_data(&key, &twitter_detail).await?;
    
    Ok(twitter_detail)
}

