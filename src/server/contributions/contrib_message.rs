use serde::{Serialize, Deserialize};
use crate::database::services::binding;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TwitterContribution {
    pub retweet: i32,
    pub quote: i32,
    pub reply: i32,
}

impl From<binding::TwitterBinding> for TwitterContribution {
    fn from(twitter_binding: binding::TwitterBinding) -> Self {
        TwitterContribution {
            retweet: twitter_binding.retweet * 1,
            quote: twitter_binding.quote* 1,
            reply: twitter_binding.comment * 1,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterResponse {
    pub user_id: String,
    pub total_interactions: i32,
    pub interaction_summary: InteractionSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionSummary {
    pub quote: i32,
    pub reply: i32,
    pub retweet: i32,
}


#[derive(Debug, serde::Deserialize)]
pub struct GetContributionsDetailRequest {
    pub media: String,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterInteraction {
    pub user_id: String,
    pub username: String,
    pub avatar_url: String,
    pub interaction_type: String,
    pub interaction_content: String,
    pub interaction_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterInteractions {
    pub media_account: String,
    pub interactions: Vec<TwitterInteraction>,
}
impl Default for TwitterInteractions {
    fn default() -> Self {
        TwitterInteractions {
            media_account: "".to_string(),
            interactions: vec![],
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TwitterDetail {
    pub project: String,
    pub contribution: String,
    pub key: i64,
}

impl From<TwitterInteraction> for TwitterDetail {
    fn from(interaction: TwitterInteraction) -> Self {
        TwitterDetail {
            project: "HETU".to_string(),
            contribution: format!("{} Hetu Twitter", interaction.interaction_type),
            key: 1,
        }
    }
}
