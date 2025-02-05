use crate::{
    common::error::AppResult,
    database::{
        dals::points,
        Storage,
    },
    helpers::redis_cache::*,
};

pub type Point = points::PointModel;

impl Storage {
    pub async fn award_points(
        &self,
        user_uid: String,
        point_type: &str,
        points: i32,
        description: &str,
    ) -> AppResult<Point> {

        let point = self.create_points(
            user_uid,
            point_type,
            points,
            description,
        ).await?;

        Ok(point)
    }

    pub async fn get_user_points(&self, user_uid: &str) -> AppResult<i64> {
        self.get_points_by_lamport_id(user_uid).await
    }

    pub async fn get_user_daily_points(&self, user_uid: &str) -> AppResult<i64> {
        self.get_daily_points_lamport_id(user_uid).await
    }

    //get points by lamport_id and point_type and description
    pub async fn get_points_by_lamport_id_and_point_type_and_description(
        &self,
        lamport_id: &str,
        point_type: &str,
        description: &str,
    ) -> AppResult<i64> {
        self.get_points_by_lamportid_pointtype_description(lamport_id, point_type, description).await
    }

    pub async fn get_count_points_by_lamport_id_and_point_type_and_description(
        &self,
        lamport_id: &str,
        point_type: &str,
        description: &str,
    ) -> AppResult<i64> {
        self.get_count_points_by_lamportid_pointtype_description(lamport_id, point_type, description).await
    }
}
