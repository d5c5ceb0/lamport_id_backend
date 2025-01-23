use crate::database::entities::vote;
use sea_orm::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateVoteRequest {
    pub data: VoteInfo,
    pub sig: String,
}

//create VoteInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VoteInfo {
    pub voter_id: Option<String>,
    pub proposal_id: String,
    pub choice: String,
    pub channel: String,
}

//impl into active model for VoteInfo
impl Into<vote::ActiveModel> for VoteInfo {
    fn into(self) -> vote::ActiveModel {
        vote::ActiveModel {
            voter_id: Set(self.voter_id.unwrap()),
            proposal_id: Set(self.proposal_id),
            choice: Set(self.choice),
            channel: Set(self.channel),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
    }
}

//impl from model for VoteInfo
impl From<vote::Model> for VoteInfo {
    fn from(model: vote::Model) -> Self {
        VoteInfo {
            voter_id: Some(model.voter_id),
            proposal_id: model.proposal_id,
            choice: model.choice,
            channel: model.channel,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GetVotesRequest {
    pub offset: i64,
    pub limit: i64,
}

#[derive(serde::Deserialize)]
pub struct GetChoiceCountRequest {
    pub proposal_id: String,
    pub choice: String,
}

