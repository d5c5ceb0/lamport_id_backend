use super::events_message::*;
use crate::{app::SharedState, common::error::AppResult, server::middlewares::AuthToken};
use axum::{debug_handler, extract::State, extract::Path, extract::Query, Json};
use std::convert::Into;

#[derive(Debug, serde::Deserialize)]
pub struct GetEventListRequest {
    pub offset: i64,
    pub limit: i64,
}


#[debug_handler]
pub async fn get_events(
    State(state): State<SharedState>,
    AuthToken(_user): AuthToken,
    Query(GetEventListRequest { offset, limit }): Query<GetEventListRequest>,
    Path(lamport_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {

    //get all events
    let events = state.store.get_all_events_by_lamport_id_order_by(&lamport_id, offset, limit).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "count": events.len(),
            "groups": events.into_iter().map(|event| {
                Event {
                    event_id: event.event_id,
                    lamport_id: event.lamport_id,
                    event_type: event.etype,
                    content: event.content,
                    created_at: event.created_at.into(),
                }
            }).collect::<Vec<_>>()
        }
    })))

    //Ok(Json(serde_json::json!({
    //    "result": {
    //        "count": 4,
    //        "groups": [
    //            //4 modck Events, event_id is uuid string, time is diffent
    //            {
    //                "event_id": "f1b9b1b0-1b1b-4b1b-8b1b-1b1b1b1b1b1b",
    //                "lamport_id": "1",
    //                "event_type": "join_hetuverse",
    //                "content": "Successfully completed the verfication process by user 001 and received their Lamport ID",
    //                "created_at": "2025-01-01T00:00:00Z"
    //            },
    //            {
    //                "event_id": "f1b9b1b0-1b1b-4b1b-8b1b-1b1b1b1b1b1b",
    //                "lamport_id": "2",
    //                "event_type": "join_dao",
    //                "content": "Join ABC as a member",
    //                "created_at": "2025-01-02T00:00:01Z"
    //            },
    //            {
    //                "event_id": "f1b9b1b0-1b1b-4b1b-8b1b-1b1b1b1b1b1b",
    //                "lamport_id": "3",
    //                "event_type": "vote",
    //                "content": "voted For on Proposal #001",
    //                "created_at": "2025-01-03T00:00:02Z"
    //            },
    //            {
    //                "event_id": "f1b9b1b0-1b1b-4b1b-8b1b-1b1b1b1b1b1b",
    //                "lamport_id": "4",
    //                "event_type": "invite",
    //                "content": "Invite a friend and earned 100 Points",
    //                "created_at": "2025-01-04T00:00:03Z"
    //            }
    //        ]
    //    }
    //})))
}
