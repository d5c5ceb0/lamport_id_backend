use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Proposal, proposal},
        Storage,
    },
};
use sea_orm::*;
use serde::Deserialize;
use uuid::Uuid;

impl Storage {
    //create create_proposal function
    pub async fn create_proposal(
        &self,
        title: String,
        description: String,
        options: Vec<String>,
        group_id: String,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<proposal::Model> {
        let model = proposal::ActiveModel {
            proposer_id: Set(Uuid::new_v4().to_string()),
            title:Set(title),
            description: Set(description),
            options: Set(options),
            created_by: Set(group_id), 
            start_time: Set(start_time.into()),
            end_time: Set(end_time.into()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let new_proposal= model.insert(self.conn.as_ref()).await?;

        Ok(new_proposal)
    }

    //get proposals list by group_id offset and limit
    pub async fn get_proposals_list_by_creator(&self, creator: String, offset: i64, limit: i64) -> AppResult<Vec<proposal::Model>> {
        Ok(Proposal::find()
            .filter(proposal::Column::CreatedBy.contains(creator))
            .order_by_asc(proposal::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

    //count proposals by creator
    pub async fn count_proposals_by_creator(&self, creator: String) -> AppResult<i64> {
        //Ok(Proposal::find()
        //    .filter(proposal::Column::CreatedBy.contains(creator))
        //    .count()
        //    .one(self.conn.as_ref())
        //    .await?
        //    .unwrap_or(0))
        //
        Ok(0)
    }

}


