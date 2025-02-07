use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Users, users},
        Storage,
    },
};
use sea_orm::prelude::Expr;
use sea_orm::*;

impl Storage {
    pub async fn create_user(&self, mut active_user: users::ActiveModel) -> AppResult<users::Model> {
        tracing::info!("user model: {:?}", active_user);

        let user_invite_code: String = active_user
            .get(users::Column::InviteCode)
            .try_as_ref()
            .ok_or(AppError::CustomError(
                "cannot get invite_code from active user".into(),
            ))?
            .to_string();

        let user_uid =  self.get_current_lamport_id().await.unwrap().to_string();

        if self
            .is_user_exists(&user_uid, &user_invite_code)
                .await?
        {
            return Err(AppError::UserExisted(format!(
                        "User: {} already exists",
                        user_uid
            )))
        }

        active_user.lamport_id = Set(user_uid.clone());

        let created_user = active_user.insert(self.conn.as_ref()).await?;
        self.increase_lamport_id().await.unwrap();

        Ok(created_user)
    }

    pub async fn is_user_exists_by_uid(&self, user_uid: &str) -> AppResult<bool> {
        let user = Users::find()
            .filter(users::Column::LamportId.eq(user_uid))
            .one(self.conn.as_ref())
            .await?;

        Ok(user.is_some())
    }

    pub async fn get_user_by_uid(&self, user_uid: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::LamportId.eq(user_uid))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                user_uid
            ))),
        }
    }

    //is_user_exists_by_username
    pub async fn is_user_exists_by_username(&self, username: &str) -> AppResult<bool> {
        let user = Users::find()
            .filter(users::Column::UserName.eq(username))
            .one(self.conn.as_ref())
            .await?;

        Ok(user.is_some())
    }
    
    //get_user_by_username
    pub async fn get_user_by_username(&self, username: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::UserName.eq(username))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                username
            ))),
        }
    }


    pub async fn is_user_exists_by_code(&self, code: &str) -> AppResult<bool> {
        let existing = Users::find()
            .filter(users::Column::InviteCode.eq(code))
            .one(self.conn.as_ref())
            .await?;

        Ok(existing.is_some())
    }

    pub async fn is_user_exists(&self, uid: &str, code: &str) -> AppResult<bool> {
        let existing = Users::find()
            .filter(
                Expr::col(users::Column::InviteCode)
                    .eq(code)
                    .or(Expr::col(users::Column::LamportId).eq(uid)),
            )
            .one(self.conn.as_ref())
            .await?;

        Ok(existing.is_some())
    }

    pub async fn get_inviter_by_code(&self, code: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::InviteCode.eq(code.to_string()))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "Inviter {} has not existed",
                code
            ))),
        }
    }

    pub async fn count_invited_users_by_uid(&self, user_uid: &str) -> AppResult<u64> {
        let user = match Users::find()
            .filter(users::Column::LamportId.eq(user_uid))
            .one(self.conn.as_ref())
            .await?
        {
            Some(u) => u,
            None => {
                return Err(AppError::UserUnExisted(format!(
                    "User: {} not exists",
                    user_uid
                )))
            }
        };

        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(user.invite_code)))
            .count(self.conn.as_ref())
            .await
            .unwrap_or(0))
    }

    pub async fn count_invited_users_by_code(&self, code: &str) -> AppResult<u64> {
        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(code.to_string())))
            .count(self.conn.as_ref())
            .await?)
    }

    pub async fn get_invited_users_by_code(&self, code: &str) -> AppResult<Vec<users::Model>> {
        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(code.to_string())))
            .all(self.conn.as_ref())
            .await?)
    }

    pub async fn count_total_users(&self) -> AppResult<u64> {
        Ok(Users::find().count(self.conn.as_ref()).await?)
    }

    //get user by address
    pub async fn get_user_by_address(&self, address: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::Address.eq(address))
            .one(self.conn.as_ref())
            .await? 
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                address
            ))),
        }
    }

    //update user's verifyed
    pub async fn update_user(&self, address: &str, verifier: &str) -> AppResult<users::Model> {
        if let Some(mut user) = Users::find()
            .filter(users::Column::Address.eq(address))
                .one(self.conn.as_ref())
                .await?
                .map(|u| u.into_active_model()) {
                    user.verified = Set(true);
                    user.verified_by = Set(Some(verifier.to_string()));
                    Ok(user.update(self.conn.as_ref()).await?)
        } else {
            Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                address
            )))
        }

    }

    pub async fn is_user_exists_by_address(&self, address: &str) -> AppResult<bool> {
        let user = Users::find()
            .filter(users::Column::Address.eq(address))
            .one(self.conn.as_ref())
            .await?;

        Ok(user.is_some())
    }

}
