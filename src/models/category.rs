use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Clone,Default)]
pub struct Category {
    pub id: Uuid,
    pub category_name: String,
    pub description: String,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}