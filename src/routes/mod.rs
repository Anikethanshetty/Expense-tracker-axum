use std::sync::Arc;
use axum::{Extension, Router, middleware};
use tower_http::trace::TraceLayer;

use crate::{AppState, middleware::auth_middleware, routes::{budget::budget_handler, category::category_handler, expense::expense_handler}};

mod auth;
mod user;
mod category;
mod budget;
mod expense;

pub fn create_router(app_state : Arc<AppState>) -> Router {
    let api_routes = Router::new()
                .nest("/auth", auth::auth_handler())

                .nest("/user", user::user_handler()
                    .layer(middleware::from_fn(auth_middleware))
                )

                .nest("/category", category_handler()
                    .layer(middleware::from_fn(auth_middleware))
                )

                .nest("/budget", budget_handler()
                    .layer(middleware::from_fn(auth_middleware))
            )
                

                .nest("/expense", expense_handler()
                    .layer(middleware::from_fn(auth_middleware))
            )
                
                .layer(TraceLayer::new_for_http())
                .layer(Extension(app_state));

    Router::new().nest("/api", api_routes)
}   