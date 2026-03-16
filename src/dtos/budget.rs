use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::Validate;
use crate::models::budget::Budget;


#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct FilterBudget {
    pub id: Option<Uuid>,
    pub amount: Option<BigDecimal>, 
    #[serde(rename="categoryId")]
    pub category_id: Option<Uuid>
}

impl FilterBudget {
    pub fn filter_budget(budget: &Budget) -> Self {
        Self { 
            id: Some(budget.id), 
            amount: Some(budget.amount.clone()),
            category_id: Some(budget.category_id)
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct BudgetData {
    pub budget: FilterBudget
}

#[derive(Debug,Serialize,Deserialize)]
pub struct BudgetResponse {
    pub status: &'static str,
    pub data: BudgetData
}


#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct CreateBudgetDto {
    #[validate(range(min=0.01,message = "Amount cannot be zero"))]
    pub amount: f64,
    #[serde(rename="categoryId")]
    pub category_id: Uuid,
}


#[derive(Debug,Deserialize,Validate)]
pub struct UpdateBudgetDto {
    #[validate(range(min=0.01,message="the amount must be greater than 0"))]
    pub amount: f64,
    pub budget_id:Uuid,
    pub category_id:Uuid
}





