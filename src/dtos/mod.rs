pub mod auth;
pub mod user;
pub mod category;
pub mod budget;
pub mod expense;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}