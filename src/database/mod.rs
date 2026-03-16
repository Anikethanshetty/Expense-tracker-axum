pub mod user;
pub mod category;
pub mod budget;
pub mod expense;
pub mod auth;
use sqlx::{Pool,Postgres};

#[derive(Debug,Clone)]
pub struct DBClient {
    pool: Pool<Postgres>
}

impl DBClient {
    pub fn new(pool:Pool<Postgres>) -> Self {
        Self {pool}
    }
}