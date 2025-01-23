use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Groups, groups},
        Storage,
    },
};
use sea_orm::*;
use uuid::Uuid;


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
    ) -> AppResult<groups::Model> {
        let new_group = groups::ActiveModel {
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
    pub async fn get_group_list(&self, offset: i64, limit: i64) -> AppResult<Vec<groups::Model>> {
        Ok(Groups::find()
            .order_by_asc(groups::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

    pub async fn get_default_group(&self) -> AppResult<groups::Model> {
        match Groups::find()
            .filter(groups::Column::GroupId.eq("293dbe4f-0b6b-462d-a778-2dceab12256b"))
            .one(self.conn.as_ref())
            .await? {
                Some(group) => Ok(group),
                None => Err(AppError::CustomError("Group has not existed".to_string())),
            }
    }

    //get only one group by creator
    pub async fn get_group_by_creator(&self, creator: &str) -> AppResult<groups::Model> {
        match Groups::find()
            .filter(groups::Column::CreatedBy.contains(creator))
            .one(self.conn.as_ref())
            .await? 
            {
                Some(group) => Ok(group),
                None => Err(AppError::CustomError(format!(
                            "Group {} has not existed",
                            creator
                ))),
            }
    }
    pub async fn get_group_by_groupid(&self, group_id: &str) -> AppResult<groups::Model> {
        match Groups::find()
            .filter(groups::Column::GroupId.eq(group_id))
            .one(self.conn.as_ref())
            .await? {
                Some(group) => Ok(group),
                None => Err(AppError::CustomError(format!(
                            "Group {} has not existed",
                            group_id
                ))),
            }
    }


}

