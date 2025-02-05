use crate::{
    common::error::{AppError, AppResult},
    database::{entities::telegram_binding, Storage},
};
use sea_orm::*; use uuid::Uuid;
use serde::{Serialize, Deserialize};

//TelegramBindingModel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramBindingModel {
    pub uid: String,
    pub lamport_id: String,
    pub telegram_id: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TelegramBindingModel {
    pub fn new(
        lamport_id: String,
        telegram_id: String,
    ) -> Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            lamport_id,
            telegram_id,
            updated_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        }
    }
}

impl From<telegram_binding::Model> for TelegramBindingModel {
    fn from(model: telegram_binding::Model) -> Self {
        Self {
            uid: model.uid,
            lamport_id: model.lamport_id,
            telegram_id: model.telegram_id,
            updated_at: model.updated_at.into(),
            created_at: model.created_at.into(),
        }
    }
}

impl From<TelegramBindingModel> for telegram_binding::ActiveModel {
    fn from(model: TelegramBindingModel) -> Self {
        Self {
            id: NotSet,
            uid: Set(model.uid),
            lamport_id: Set(model.lamport_id),
            telegram_id: Set(model.telegram_id),
            updated_at: Set(model.updated_at.into()),
            created_at: Set(model.created_at.into()),
        }
    }
}


impl Storage {
    pub async fn create_telegram_binding(&self, model: TelegramBindingModel) -> AppResult<TelegramBindingModel> {
        let model = telegram_binding::ActiveModel::from(model);
        let model = model.insert(self.conn.as_ref()).await?;
        Ok(model.into())
    }

    //get contribution by lamport_id
    pub async fn get_telegram_binding_by_lamport_id(
        &self,
        lamport_id: &str,
    ) -> AppResult<TelegramBindingModel> {
        let model = telegram_binding::Entity::find()
            .filter(telegram_binding::Column::LamportId.contains(lamport_id))
            .one(self.conn.as_ref())
            .await?.ok_or(AppError::CustomError("twitter not found".to_string()))?;

        Ok(model.into())
    }

}
