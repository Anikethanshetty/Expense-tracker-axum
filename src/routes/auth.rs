use axum::{Router, routing::post};
use crate::handler::auth::{login,register};

pub fn auth_handler() ->Router {
    Router::new()
    .route("/register", post(register))
    .route("/login", post(login))
}