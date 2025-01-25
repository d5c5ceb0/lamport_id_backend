use crate::{
    common::error::{AppResult,AppError},
    database::{
        entities::{prelude::Proposals, proposals},
        Storage,
    },
};
use sea_orm::*;
use uuid::Uuid;

impl Storage {
    //create create_proposal function
    pub async fn create_proposal(
        &self,
        title: String,
        description: String,
        options: Vec<String>,
        creator: String,
        group_id: String,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<proposals::Model> {
        let model = proposals::ActiveModel {
            proposal_id: Set(Uuid::new_v4().to_string()),
            title:Set(title),
            description: Set(description),
            options: Set(options),
            group_id: Set(group_id),
            created_by: Set(creator), 
            start_time: Set(start_time.into()),
            end_time: Set(end_time.into()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let new_proposal= model.insert(self.conn.as_ref()).await?;

        Ok(new_proposal)
    }

    //get proposals list by creator offset and limit
    pub async fn get_proposals_list_by_creator(&self, creator: String, offset: i64, limit: i64) -> AppResult<Vec<proposals::Model>> {
        Ok(Proposals::find()
            .filter(proposals::Column::CreatedBy.contains(creator))
            .order_by_asc(proposals::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

    //count proposals by group_id
    pub async fn count_proposals_by_groupid(&self, group_id: &str) -> AppResult<u64> {
        Ok(Proposals::find()
            .filter(proposals::Column::GroupId.contains(group_id))
            .count(self.conn.as_ref())
            .await?)
    }

    pub async fn get_proposals_list_by_groupid(&self, group_id: &str, offset: i64, limit: i64) -> AppResult<Vec<proposals::Model>> {
        Ok(Proposals::find()
            .filter(proposals::Column::GroupId.contains(group_id))
            .order_by_asc(proposals::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

    pub async fn get_proposals_list_with_votes_by_groupid(&self, group_id: &str, offset: i64, limit: i64) -> AppResult<Vec<(proposals::Model, u64)>> {
        let proposals = Proposals::find()
            .filter(proposals::Column::GroupId.contains(group_id))
            .order_by_asc(proposals::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?;

        let mut result = Vec::new();
        for proposal in proposals {
            //votes
            let vote_count = self.count_votes_by_proposal_id(&proposal.proposal_id).await?;
            result.push((proposal, vote_count));
        }


        Ok(result)
    }

    //get_proposals_list_with_votes_by_groupid with order asc/desc, status: Passed or active or all
    pub async fn get_proposals_list_with_votes_by_groupid_order_by(
        &self,
        group_id: &str,
        offset: i64,
        limit: i64,
        order_by: &str,
        status: Option<String>,
    ) -> AppResult<Vec<(proposals::Model, u64)>> {
        let mut query = Proposals::find()
            .filter(proposals::Column::GroupId.contains(group_id))
            .offset(offset as u64)
            .limit(limit as u64);

        if let Some(s) = status {
            if s == "Passed" {
                query = query.filter(proposals::Column::EndTime.lt(chrono::Utc::now()));
            } else if s == "Active" {
                query = query.filter(proposals::Column::EndTime.gt(chrono::Utc::now()));
            }
        }

        if order_by == "asc" {
            query = query.order_by_asc(proposals::Column::CreatedAt);
        } else {
            query = query.order_by_desc(proposals::Column::CreatedAt);
        }

        let proposals = query.all(self.conn.as_ref()).await?;

        let mut result = Vec::new();
        for proposal in proposals {
            //votes
            let vote_count = self.count_votes_by_proposal_id(&proposal.proposal_id).await?;
            result.push((proposal, vote_count));
        }

        Ok(result)
    }


    pub async fn get_proposal_by_proposal_id(&self, proposal_id: &str) -> AppResult<proposals::Model> {
        match Proposals::find()
            .filter(proposals::Column::ProposalId.eq(proposal_id))
            .one(self.conn.as_ref())
            .await? {
                Some(proposal) => Ok(proposal),
                None => Err(AppError::CustomError(format!(
                            "Proposal {} has not existed",
                            proposal_id
                ))),
            }
    }

}


