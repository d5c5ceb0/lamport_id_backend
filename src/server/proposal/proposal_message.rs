use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::{self,State, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::database::entities::proposal;


#[derive(Deserialize, Serialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,  //check For, Against, Abstain
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
}

// ProposalInfo struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalInfo {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub created_by: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

//impl from for ProposalInfo
impl From<proposal::Model> for ProposalInfo {
    fn from(proposal: proposal::Model) -> Self {
        ProposalInfo {
            title: proposal.title,
            description: proposal.description,
            options: proposal.options,
            created_by: proposal.created_by,
            start_time: proposal.start_time.into(),
            end_time: proposal.end_time.into(),
            created_at: proposal.created_at.into(),
            updated_at: proposal.updated_at.into(),
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct GetProposalsRequest {
    //pub creator: String,
    pub offset: i64,
    pub limit: i64,
}

