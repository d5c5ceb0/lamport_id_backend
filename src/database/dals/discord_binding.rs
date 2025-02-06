use crate::{
    common::error::{AppError, AppResult},
    database::{entities::discord_binding, Storage},
};
use sea_orm::*; use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordBindingModel {
    pub uid: String,
    pub lamport_id: String,
    pub discord_id: String,
    pub user_name: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl DiscordBindingModel {
    pub fn new(
        lamport_id: String,
        discord_id: String,
        user_name: String,
    ) ->Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            lamport_id,
            discord_id,
            user_name,
            updated_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        }
    }
}

impl From<DiscordBindingModel> for discord_binding::ActiveModel {
    fn from(model: DiscordBindingModel) -> Self {
        Self {
            id: NotSet,
            uid: Set(model.uid),
            lamport_id: Set(model.lamport_id),
            discord_id: Set(model.discord_id),
            user_name: Set(model.user_name),
            updated_at: Set(model.updated_at.into()),
            created_at: Set(model.created_at.into()),
        }
    }
}

impl From<discord_binding::Model> for DiscordBindingModel {
    fn from(model: discord_binding::Model) -> Self {
        Self {
            uid: model.uid,
            lamport_id: model.lamport_id,
            discord_id: model.discord_id,
            user_name: model.user_name,
            updated_at: model.updated_at.into(),
            created_at: model.created_at.into()
        }
    }
}


impl Storage {
    pub async fn create_discord_binding(&self, model: DiscordBindingModel) -> AppResult<DiscordBindingModel> {
        let model = discord_binding::ActiveModel::from(model);
        let model = model.insert(self.conn.as_ref()).await?;
        Ok(model.into())
    }

    //get contribution by lamport_id
    pub async fn get_discord_binding_by_lamport_id(
        &self,
        lamport_id: &str,
    ) -> AppResult<DiscordBindingModel> {
        let model = discord_binding::Entity::find()
            .filter(discord_binding::Column::LamportId.contains(lamport_id))
            .one(self.conn.as_ref())
            .await?.ok_or(AppError::CustomError("discord not found".to_string()))?;

        Ok(model.into())
    }

}
