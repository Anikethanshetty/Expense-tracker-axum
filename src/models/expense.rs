use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::{ NaiveDate};
use sqlx::types::Uuid;

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Default)]
pub struct Expense {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub expense_date: NaiveDate,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

