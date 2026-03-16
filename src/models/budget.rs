use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Default)]
pub struct Budget {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}