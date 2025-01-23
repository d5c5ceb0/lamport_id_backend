use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::prelude::LamportId,
        Storage,
    },
};
use sea_orm::*;


impl Storage {
    pub async fn get_current_lamport_id(&self) -> AppResult<i64> {
        match LamportId::find().one(self.conn.as_ref()).await? {
            Some(last) => Ok(last.current_value),
            None => Err(AppError::CustomError("LamportId has not existed".into()))
        }
    }

    pub async fn increase_lamport_id(&self) -> AppResult<i64> {
        if let Some(last) = LamportId::find().one(self.conn.as_ref()).await? {
            let mut last_mut = last.clone().into_active_model();
            last_mut.current_value = Set(last.current_value + 1_i64);
            last_mut.updated_at = Set(chrono::Utc::now().into());
            last_mut.update(self.conn.as_ref()).await?;
            Ok(last.current_value)
        } else {
            Err(AppError::CustomError("LamportId has not existed".into()))
        }
    }
}
