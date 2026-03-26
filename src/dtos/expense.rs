use bigdecimal::BigDecimal;
use chrono::{ Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::{Validate, ValidationError};
use chrono::NaiveDate;

use crate::models::expense::Expense;


#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
#[serde(rename_all="camelCase")]
pub struct FilterExpense {
    pub id: Uuid,
    pub amount: BigDecimal,  
    pub category_id:Uuid,
    pub expense_date: NaiveDate
}

impl FilterExpense {
    pub fn filter_expense(expense: &Expense) -> Self {
        Self { 
            id: expense.id, 
            amount: expense.amount.clone(),
            category_id: expense.category_id,
            expense_date: expense.expense_date
        }
    }
}

fn validate_expense_date(date: &NaiveDate) -> Result<(), ValidationError> {
    if *date > Utc::now().date_naive() {
        return Err(ValidationError::new("future_date"))
    }
    Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ExpenseData {
    pub expense: FilterExpense
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ExpenseResponseDto {
    pub status: &'static str,
    pub data: ExpenseData
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ExpensesResponseDto {
    pub status: &'static str,
    pub data: ExpensesData
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ExpensesData {
    pub expense: Vec<FilterExpense> 
}



#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct CreateExpenseDto {
    #[validate(range(min = 0.01))]
    pub amount: f64,
    #[serde(rename = "categoryId")]
    pub category_id: Uuid,
    #[serde(rename = "expenseDate")]
    #[validate(custom(function="validate_expense_date"))]
    pub expenses_date: NaiveDate
}




#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct GetExpensesQuery {
    #[validate(range(min=1,message="page must be greater than or equal to one"))]
    pub page: Option<i64>,
    # [validate(range(min=1,max=100,message="limit must greater than 1 or equal to 1 and less than 100"))]
    pub limit: Option<i64>
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct UpdateExpenseDto {
    #[validate(range(min=0.01,message="amount must be greater 0"),)]
    pub amount: Option<f64>,
    #[serde(rename="expenseDate")]
    #[validate(custom(function="validate_expense_date"))]
    pub expense_date: Option<NaiveDate>,
     #[serde(rename = "categoryId")]
    pub category_id: Uuid,
     #[serde(rename = "expenseId")]
    pub expense_id: Uuid,
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct DeleteExpenseDto {
    #[serde(rename="categorryId")]
    pub category_id:Uuid,
    #[serde(rename="expenseId")]
    pub expense_id: Uuid,
}