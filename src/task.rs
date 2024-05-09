use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

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