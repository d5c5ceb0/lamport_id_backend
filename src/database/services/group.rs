use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Group, group},
        Storage,
    },
};
use sea_orm::*;
use uuid::Uuid;

use super::proposal;

impl Storage {
    //create group
    pub async fn create_group(
        &self,
        name: String,
        logo: String,
        description: Option<String>,
        website: String,
        twitter: String,
        creator_id: String,
    ) -> AppResult<group::Model> {
        let new_group = group::ActiveModel {
            group_id: Set(Uuid::new_v4().to_string()),
            name: Set(name),
            logo: Set(logo),
            description: Set(description),
            website: Set(website),
            twitter: Set(twitter),
            created_by: Set(creator_id),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        Ok(new_group.insert(self.conn.as_ref()).await?)
    }

    //get all group list offset and limit
    pub async fn get_group_list(&self, offset: i64, limit: i64) -> AppResult<Vec<group::Model>> {
        Ok(Group::find()
            .order_by_asc(group::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

    //get only one group by creator
    pub async fn get_group_by_creator(&self, creator: &str) -> AppResult<group::Model> {
        match Group::find()
            .filter(group::Column::CreatedBy.contains(creator))
            .one(self.conn.as_ref())
            .await? 
            {
                Some(proposals) => Ok(proposals),
                None => Err(AppError::CustomError(format!(
                            "Group {} has not existed",
                            creator
                ))),
            }
    }


}

