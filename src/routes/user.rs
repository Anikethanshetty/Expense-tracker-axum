use axum::{Router, routing::post};
use crate::handler::{user::{update_user,delete_user}};

pub fn user_handler() -> Router {
    Router::new()
    .route("/update", post(update_user))
    .route("/delete", post(delete_user))
}