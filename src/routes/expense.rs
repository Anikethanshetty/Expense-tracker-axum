use axum::{Router, routing::{delete, get, post}};

use crate::handler::expense::{create_expense, get_all_expenses,get_expense,delete_expense,update_expense};


pub fn expense_handler() ->Router {
    Router::new()
    .route("/create", post(create_expense))
    .route("/get/all/{category_id}", get(get_all_expenses))
    .route("/get/{category_id}/{expense_id}", get(get_expense))
    .route("/update", post(update_expense))
    .route("/delete", delete(delete_expense))
}