use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

//TODO: set range for priority,

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub priority: u8,
    pub status: Status,
    pub is_cancelled: bool,
    pub deadline: DateTime<Utc>,
}
