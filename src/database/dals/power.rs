use crate::{
    common::error::AppResult,
    database::{
        entities::{power, prelude::Power},
        Storage,
    },
};
use sea_orm::*;

#[derive(FromQueryResult, Debug)]
struct AggregationResult {
    total_points: Option<i64>, // Match the alias name
}

impl Storage {
    pub async fn create_energy(
        &self,
        user_uid: String,
        power_type: &str,
        amounts: i32,
    ) -> AppResult<power::Model> {
        let point_entry = power::ActiveModel {
            lamport_id: Set(user_uid),
            types: Set(power_type.to_owned()),
            amounts: Set(amounts),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let point = point_entry.insert(self.conn.as_ref()).await?;

        Ok(point)
    }

    pub async fn get_user_power(&self, user_uid: &str) -> AppResult<i64> {
        match Power::find()
            .filter(power::Column::LamportId.eq(user_uid))
            .select_only()
            .column_as(power::Column::Amounts.sum(), "total_points")
            .into_model::<AggregationResult>()
            .one(self.conn.as_ref())
            .await?
        {
            Some(aggr_result) => Ok(aggr_result.total_points.unwrap_or(0)),
            None => Ok(0),
        }
    }
}
