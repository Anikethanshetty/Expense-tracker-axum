use axum::{Router, routing::{delete, get, post}};
use crate::handler::budget::{create_budget, delete_budget, get_budget, update_budget};

pub fn budget_handler()-> Router {
    Router::new()
    .route("/create", post(create_budget))
    .route("/{category_id}", get(get_budget))
    .route("/update", post(update_budget))
    .route("/{category_id}/{budget_id}", delete(delete_budget))
}