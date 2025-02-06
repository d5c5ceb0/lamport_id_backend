use crate::{
    common::error::{AppError, AppResult},
    database::{entities::github_binding, Storage},
};
use sea_orm::*; use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubBindingModel {
    pub uid: String,
    pub lamport_id: String,
    pub github_id: String,
    pub user_name: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl GithubBindingModel {
    pub fn new(
        lamport_id: String,
        github_id: String,
        user_name: String,
    ) ->Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            lamport_id,
            github_id,
            user_name,
            updated_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        }
    }
}

impl From<GithubBindingModel> for github_binding::ActiveModel {
    fn from(model: GithubBindingModel) -> Self {
        Self {
            id: NotSet,
            uid: Set(model.uid),
            lamport_id: Set(model.lamport_id),
            github_id: Set(model.github_id),
            user_name: Set(model.user_name),
            updated_at: Set(model.updated_at.into()),
            created_at: Set(model.created_at.into()),
        }
    }
}

impl From<github_binding::Model> for GithubBindingModel {
    fn from(model: github_binding::Model) -> Self {
        Self {
            uid: model.uid,
            lamport_id: model.lamport_id,
            github_id: model.github_id,
            user_name: model.user_name,
            updated_at: model.updated_at.into(),
            created_at: model.created_at.into()
        }
    }
}


impl Storage {
    pub async fn create_github_binding(&self, model: GithubBindingModel) -> AppResult<GithubBindingModel> {
        let model = github_binding::ActiveModel::from(model);
        let model = model.insert(self.conn.as_ref()).await?;
        Ok(model.into())
    }

    //get contribution by lamport_id
    pub async fn get_github_binding_by_lamport_id(
        &self,
        lamport_id: &str,
    ) -> AppResult<GithubBindingModel> {
        let model = github_binding::Entity::find()
            .filter(github_binding::Column::LamportId.contains(lamport_id))
            .one(self.conn.as_ref())
            .await?.ok_or(AppError::CustomError("github not found".to_string()))?;

        Ok(model.into())
    }

}
