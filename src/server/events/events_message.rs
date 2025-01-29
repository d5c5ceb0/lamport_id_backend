use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub lamport_id: String,
    pub event_type: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
