use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::Validate;

use crate::{dtos::{budget::{self, FilterBudget}, category::{self, FilterCategory}, expense::FilterExpense}, models::{budget::Budget, category::Category, expense::Expense, user::User}};


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FilterUserDto {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub categories: Option<Vec<FilterCategory>>,
    pub budgets: Option<Vec<FilterBudget>>,
    pub expenses: Option<Vec<FilterExpense>>
} 

impl FilterUserDto {
    pub fn filter_user(user: &User,categories:Option<&Vec<Category>>,budgets:Option<&Vec<Budget>>,expenses:Option<&Vec<Expense>>) -> Self {
        let mut filtered_categories: Vec<FilterCategory> = Vec::new();
        let mut filtered_budgets: Vec<FilterBudget> = Vec::new();
        let mut filtered_expenses: Vec<FilterExpense> = Vec::new();


        if let Some(categories_vec) = categories {
            for category in categories_vec {
                let res = FilterCategory::filter_category(category);
                filtered_categories.push(res);
            }
        }

        if let Some(budgets_vec) = budgets {
            for budget in budgets_vec {
                let res = FilterBudget::filter_budget(budget);
                filtered_budgets.push(res);
            }
        }

        if let Some(expenses_vec) = expenses {
            for expense in expenses_vec {
                let res = FilterExpense::filter_expense(expense);
                filtered_expenses.push(res);
            }
        }
       
        Self { 
            id:Some(user.id) , 
            username: Some(user.username.clone()), 
            email: Some(user.email.clone()),
            categories: Some(filtered_categories), 
            budgets: Some(filtered_budgets), 
            expenses: Some(filtered_expenses) 
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct UserData {
    pub user: FilterUserDto
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct UserResponse {
    pub status: &'static str,
    pub data: UserData
}

#[derive(Debug,Deserialize,Validate)]
pub struct UpdateUserDto {
    #[validate(length(min=1,max=255,message="name greater than or equal to 1 and less than 255 characters"))]
    pub username: Option <String>,
    #[validate(length(min=1,message = "email is required"),
        email(message = "Email is invalid"))]
    pub email: Option<String>
}

#[derive(Debug,Deserialize,Validate,Serialize)]
pub struct DeleteUserDto {
    #[validate(length(min=1,message = "email is required"),
        email(message = "Email is invalid"))]
    pub email: String
}
