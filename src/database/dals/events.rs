use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Events, events},
        Storage,
    },
};
use sea_orm::*;
use uuid::Uuid;

impl Storage {
    //create events
    pub async fn create_event(
        &self,
        lamport_id: String,
        etype: String,
        content: String,
    ) -> AppResult<events::Model> {
        let event : events::ActiveModel = events::ActiveModel {
            event_id : Set(Uuid::new_v4().to_string()),
            lamport_id : Set(lamport_id),
            etype : Set(etype),
            content : Set(content),
            created_at : Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        Ok(event.insert(self.conn.as_ref()).await?)
    }

    //get event by event id
    pub async fn get_event_by_event_id(&self, event_id: &str) -> AppResult<events::Model> {
        match Events::find()
            .filter(events::Column::EventId.eq(event_id))
            .one(self.conn.as_ref())
            .await? {
                Some(event) => Ok(event),
                None => Err(AppError::CustomError("Event has not existed".to_string())),
            }
    }

    //get all events
    pub async fn get_all_events(&self) -> AppResult<Vec<events::Model>> {
        Ok(Events::find().all(self.conn.as_ref()).await?)
    }

    //get all events by lamport id
    pub async fn get_all_events_by_lamport_id(&self, lamport_id: &str) -> AppResult<Vec<events::Model>> {
        Ok(Events::find()
            .filter(events::Column::LamportId.eq(lamport_id))
            .all(self.conn.as_ref())
            .await?)
    }

    //get all events by type
    pub async fn get_all_events_by_type(&self, etype: &str) -> AppResult<Vec<events::Model>> {
        Ok(Events::find()
            .filter(events::Column::Etype.eq(etype))
            .all(self.conn.as_ref())
            .await?)
    }

    //offset and limit
    pub async fn get_all_events_by_lamport_id_order_by(&self, lamport_id: &str, offset: i64, limit: i64) -> AppResult<Vec<events::Model>> {
        Ok(Events::find()
            .filter(events::Column::LamportId.eq(lamport_id))
            .order_by_asc(events::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

}
