use axum::{Router, handler::{Handler, Layered}, http::{HeaderValue, Method, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}}, response::IntoResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

use crate::{config::Config, database::DBClient, routes::create_router};
use std::{self, sync::Arc};

mod config;
mod models;
mod dtos;
mod error;
mod database;
mod routes;
mod middleware;
mod utils;
mod handler;

#[derive(Debug,Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client:DBClient,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::OPTIONS])
        .allow_credentials(true);

    let db_client = DBClient::new(pool);

    let app_state = Arc::new(AppState {
        db_client,
        env: config.clone(),
    });

    let app = create_router(app_state).layer(cors);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    println!("Server running on port {}", config.port);

    axum::serve(listener, app).await.unwrap();
}