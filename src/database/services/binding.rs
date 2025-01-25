use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::TwitterBinding, twitter_binding},
        Storage,
    },
};
use sea_orm::*;
use uuid::Uuid;


impl Storage {
    //create twitter binding
    pub async fn binding_twitter(
        &self,
        user_id: String,
        x_id: String,
        name: String,
        user_name: String,
        image_url: String,
        access_token: String,
        refresh_token: String,
        token_type: String,
        scope: String,
    ) -> AppResult<twitter_binding::Model> {
        let binding : twitter_binding::ActiveModel = twitter_binding::ActiveModel {
            user_id : Set(user_id),
            x_id : Set(x_id),
            name : Set(name),
            user_name : Set(user_name),
            image_url : Set(image_url),
            access_token : Set(access_token),
            refresh_token : Set(refresh_token),
            token_type : Set(token_type),
            scope : Set(scope),
            ..Default::default()
        };

        Ok(binding.insert(self.conn.as_ref()).await?)
    }

    //get twitter binding by user id
    pub async fn get_twitter_binding_by_user_id(&self, user_id: &str) -> AppResult<twitter_binding::Model> {
        match TwitterBinding::find()
            .filter(twitter_binding::Column::UserId.eq(user_id))
            .one(self.conn.as_ref())
            .await? {
                Some(binding) => Ok(binding),
                None => Err(AppError::CustomError("Twitter binding has not existed".to_string())),
            }
    }

}

