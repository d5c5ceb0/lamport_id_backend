use crate::{
    common::error::{AppError, AppResult},
    database::{entities::twitter_binding, Storage},
};
use sea_orm::*; use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterBindingModel {
    pub uid: String,
    pub lamport_id: String,
    pub x_id: String,
    pub name: String,
    pub user_name: String,
    pub image_url: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub scope: String,
    pub retweet: i32,
    pub mention: i32,
    pub comment: i32,
    pub quote: i32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TwitterBindingModel {
    pub fn new(
        lamport_id: String,
        x_id: String,
        name: String,
        user_name: String,
        image_url: String,
        access_token: String,
        refresh_token: String,
        token_type: String,
        scope: String,
    ) ->Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            lamport_id,
            x_id,
            name,
            user_name,
            image_url,
            access_token,
            refresh_token,
            token_type,
            scope,
            retweet: 0,
            quote: 0,
            mention: 0,
            comment: 0,
            updated_at: chrono::Utc::now(),
        }
    }
}

impl From<TwitterBindingModel> for twitter_binding::ActiveModel {
    fn from(model: TwitterBindingModel) -> Self {
        Self {
            id: NotSet,
            uid: Set(model.uid),
            lamport_id: Set(model.lamport_id),
            x_id: Set(model.x_id),
            name: Set(model.name),
            user_name: Set(model.user_name),
            image_url: Set(model.image_url),
            access_token: Set(model.access_token),
            refresh_token: Set(model.refresh_token),
            token_type: Set(model.token_type),
            scope: Set(model.scope),
            retweet: Set(model.retweet),
            quote: Set(model.quote),
            mention: Set(model.mention),
            comment: Set(model.comment),
            updated_at: Set(model.updated_at.into()),
            created_at: Set(chrono::Utc::now().into()),
        }
    }
}

impl From<twitter_binding::Model> for TwitterBindingModel {
    fn from(model: twitter_binding::Model) -> Self {
        Self {
            uid: model.uid,
            lamport_id: model.lamport_id,
            x_id: model.x_id,
            name: model.name,
            user_name: model.user_name,
            image_url: model.image_url,
            access_token: model.access_token,
            refresh_token: model.refresh_token,
            token_type: model.token_type,
            scope: model.scope,
            retweet: model.retweet,
            quote: model.quote,
            mention: model.mention,
            comment: model.comment,
            updated_at: model.updated_at.into()
        }
    }
}


impl Storage {
    pub async fn create_twitter_binding(&self, model: TwitterBindingModel) -> AppResult<TwitterBindingModel> {
        let model = twitter_binding::ActiveModel::from(model);
        let model = model.insert(self.conn.as_ref()).await?;
        Ok(model.into())
    }

    //update contribution by lamport_id
    pub async fn update_twitter_binding_by_lamport_id(
        &self,
        lamport_id: &str,
        retweet: i32,
        mention: i32,
        comment: i32,
        quote: i32,
    ) -> AppResult<TwitterBindingModel> {
        if let Some(mut model) = twitter_binding::Entity::find()
            .filter(twitter_binding::Column::LamportId.contains(lamport_id))
            .one(self.conn.as_ref())
            .await?.map(|m| m.into_active_model()) {
                model.retweet = Set(retweet);
                model.mention = Set(mention);
                model.comment = Set(comment);
                model.quote = Set(quote);
                model.updated_at = Set(chrono::Utc::now().into());

                Ok(model.update(self.conn.as_ref()).await?.into())
        } else {
            Err(AppError::CustomError("twitter not found".to_string()))
        }
    }

    //get contribution by lamport_id
    pub async fn get_twitter_binding_by_lamport_id(
        &self,
        lamport_id: &str,
    ) -> AppResult<TwitterBindingModel> {
        let model = twitter_binding::Entity::find()
            .filter(twitter_binding::Column::LamportId.contains(lamport_id))
            .one(self.conn.as_ref())
            .await?.ok_or(AppError::CustomError("twitter not found".to_string()))?;

        Ok(model.into())
    }

}
