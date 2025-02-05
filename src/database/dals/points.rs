use crate::{
    common::error::AppResult,
    database::{
        entities::{points, prelude::Points},
        Storage,
    },
};
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Debug)]
pub struct AggregationResult {
    total_points: Option<i64>, // Match the alias name
}

//a struct from points::model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointModel {
    pub lamport_id: String,
    pub point_type: String,
    pub amounts: i32,
    pub description: Option<String>,
}

impl From<points::Model> for PointModel {
    fn from(point: points::Model) -> Self {
        PointModel {
            lamport_id: point.lamport_id,
            point_type: point.point_type,
            amounts: point.amounts,
            description: point.description,
        }
    }
}

impl From<PointModel> for points::ActiveModel {
    fn from(point: PointModel) -> Self {
        points::ActiveModel {
            id: NotSet,
            lamport_id: Set(point.lamport_id),
            point_type: Set(point.point_type),
            amounts: Set(point.amounts),
            description: Set(point.description),
            created_at: Set(chrono::Utc::now().into()),
            expires_at: Set(None),
        }
    }
}

impl Storage {
    pub async fn create_points(
        &self,
        user_uid: String,
        point_type: &str,
        points: i32,
        description: &str,
    ) -> AppResult<PointModel> {
        let point_entry = points::ActiveModel {
            lamport_id: Set(user_uid),
            point_type: Set(point_type.to_owned()),
            amounts: Set(points),
            description: Set(Some(description.to_owned())),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let point = point_entry.insert(self.conn.as_ref()).await?;

        Ok(point.into())
    }

    pub async fn get_points_by_lamport_id(&self, user_uid: &str) -> AppResult<i64> {
        match Points::find()
            .filter(points::Column::LamportId.eq(user_uid))
            .select_only()
            .column_as(points::Column::Amounts.sum(), "total_points")
            .into_model::<AggregationResult>()
            .one(self.conn.as_ref())
            .await?
        {
            Some(aggr_result) => Ok(aggr_result.total_points.unwrap_or(0)),
            None => Ok(0),
        }
    }
    pub async fn cleanup_expired_point(&self) -> AppResult<()> {
        use sea_orm::EntityTrait;

        points::Entity::delete_many()
            .filter(points::Column::ExpiresAt.lt(chrono::Utc::now()))
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }

    pub async fn get_daily_points_lamport_id(&self, user_uid: &str) -> AppResult<i64> {
        let today = chrono::Utc::now().date_naive();
        match Points::find()
            .filter(points::Column::LamportId.eq(user_uid))
            .filter(points::Column::CreatedAt.gt(today.and_hms_opt(0, 0, 0)))
            .select_only()
            .column_as(points::Column::Amounts.sum(), "total_points")
            .into_model::<AggregationResult>()
            .one(self.conn.as_ref())
            .await?
        {
            Some(aggr_result) => Ok(aggr_result.total_points.unwrap_or(0)),
            None => Ok(0),
        }
    }

    //get points by lamport_id and point_type and description
    pub async fn get_points_by_lamportid_pointtype_description(
        &self,
        lamport_id: &str,
        point_type: &str,
        description: &str,
    ) -> AppResult<i64> {
        match Points::find()
            .filter(points::Column::LamportId.eq(lamport_id))
            .filter(points::Column::PointType.eq(point_type))
            .filter(points::Column::Description.eq(description))
            .select_only()
            .column_as(points::Column::Amounts.sum(), "total_points")
            .into_model::<AggregationResult>()
            .one(self.conn.as_ref())
            .await?
        {
            Some(aggr_result) => Ok(aggr_result.total_points.unwrap_or(0)),
            None => Ok(0),
        }
    }

    //get count of points by lamport_id and point_type and description
    pub async fn get_count_points_by_lamportid_pointtype_description(
        &self,
        lamport_id: &str,
        point_type: &str,
        description: &str,
    ) -> AppResult<i64> {
        let count = Points::find()
            .filter(points::Column::LamportId.eq(lamport_id))
            .filter(points::Column::PointType.eq(point_type))
            .filter(points::Column::Description.eq(description))
            .count(self.conn.as_ref())
            .await?;

        Ok(count as i64)
    }
}
