use serde::{Deserialize, Serialize};
use crate::{database::entities::proposals,server::proposal::proposal_service::get_proposal_status};


#[derive(Deserialize, Serialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,  //check For, Against, Abstain
    pub group_id: String,
    //pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
}

// ProposalInfo struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalInfo {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub created_by: String,
    pub group_id: String,
    pub proposal_id: String,
    pub status: String,
    pub ai_comments:String,
    pub votes: u64,
    pub time_left: String,
    pub ai_stats_participation: String,
    pub ai_stats_weight: String,
    pub earn: u64,
    pub contribution: u64,
    //pub start_time: chrono::DateTime<chrono::Utc>,
    //pub end_time: chrono::DateTime<chrono::Utc>,
    //pub created_at: chrono::DateTime<chrono::Utc>,
    //pub updated_at: chrono::DateTime<chrono::Utc>,
}

//impl from for ProposalInfo
impl From<proposals::Model> for ProposalInfo {
    fn from(proposal: proposals::Model) -> Self {
        ProposalInfo {
            title: proposal.title,
            description: proposal.description,
            options: proposal.options,
            created_by: proposal.created_by,
            group_id: proposal.group_id,
            proposal_id: proposal.proposal_id,
            status: get_proposal_status(proposal.start_time.into(), proposal.end_time.into()),
            ai_comments: "".to_string(),
            votes: 0,
            time_left: proposal.end_time.signed_duration_since(chrono::Utc::now()).num_seconds().to_string(),
            ai_stats_participation: "0.5".to_string(),
            ai_stats_weight: "0.3".to_string(),
            earn: 0,
            contribution: 0,
            //start_time: proposal.start_time.into(),
            //end_time: proposal.end_time.into(),
            //created_at: proposal.created_at.into(),
            //updated_at: proposal.updated_at.into(),
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct GetProposalsRequest {
    pub offset: i64,
    pub limit: i64,
}


