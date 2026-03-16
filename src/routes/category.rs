use axum::{Router, routing::{delete, get, post}};

use crate::{
        handler::category::{
        create_category, 
        get_all_categories, 
        get_category, 
        delete_category,
        update_categoy
    }, 
};


pub fn category_handler() -> Router {
    Router::new()
    .route("/create", post(create_category))
    .route("/get/all", get(get_all_categories))
    .route("/get/:category_id", get(get_category))
    .route("/update", post(update_categoy))
    .route("/delete/:category_id", delete(delete_category))
}