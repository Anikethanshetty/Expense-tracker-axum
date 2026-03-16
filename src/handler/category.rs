use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, Query}, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, database::category::CategoryExt, dtos::{Response, category::{CategoriesData, CategoriesResponseDto, CategoryData, CategoryResponseDto, CreateCategoryDto, FilterCategory, GetCategoriesQuery, UpdateCategoryDto}}, error::{ErrorMessage, HttpError}, middleware::JwtAuthMiddleware};


pub async fn create_category(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body) : Json<CreateCategoryDto>
) -> Result<impl IntoResponse,HttpError> {
    body.validate().map_err(|e| {
        HttpError::bad_request(e.to_string())
    })?;

    let category = app_state
                        .db_client
                        .create_category(
                            body.category_name, 
                            body.description, 
                            user.user.id
                        ).await;


        match category {
            Ok(category) => {
                 Ok((
                    StatusCode::CREATED,
                    Json(
                        CategoryResponseDto {
                            status : "success",
                            data : CategoryData {
                                category : FilterCategory::filter_category(&category)
                            }
                        }
                    )
                ))
            },
            Err(sqlx::Error::Database(db_error)) => {
                if db_error.is_unique_violation() {
                     Err(HttpError::unique_constraint_violation(db_error.to_string()))

                }
                else {
                     Err(HttpError::server_error(ErrorMessage::ServerError.to_string()))  
                }
            }

            Err(e) => {
                Err(HttpError::server_error(e.to_string()))
            }
        }

}

pub async fn get_all_categories(
    Query(query_params) : Query<GetCategoriesQuery>,
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
) -> Result<impl IntoResponse,HttpError> {

    query_params.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;
    
    let categories = app_state
                            .db_client
                            .get_all_categories(
                                user.user.id, 
                                query_params.page, 
                                query_params.limit
                            ).await
                            .map_err(|e| {
                                HttpError::server_error(e.to_string())
                            })?.unwrap_or_default();
            
        let filtered_categories : Vec<FilterCategory> = categories.iter()
                                                    .map(FilterCategory::filter_category)
                                                    .collect();

          Ok((
            StatusCode::OK,
            Json(
                CategoriesResponseDto {
                    status : "success",
                    data : CategoriesData {
                        categories : filtered_categories
                    }
                }
            )
          ))
}

pub async fn get_category(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Path(category_id) : Path<Uuid>
) -> Result<impl IntoResponse,HttpError> {

    let category = app_state
                            .db_client
                            .get_category(user.user.id, category_id)
                            .await
                            .map_err(|e| {
                                HttpError::server_error(e.to_string())
                            })?.unwrap_or_default();
                
                      
        Ok((
            StatusCode::FOUND,
            Json(
                CategoryResponseDto {
                    status : "success",
                    data : CategoryData { 
                        category: FilterCategory::filter_category(&category)
                     }
                }
            )
        ))
}

pub async fn update_categoy(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body): Json<UpdateCategoryDto>
) -> Result<impl IntoResponse,HttpError> {

    body.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;

    let updated_category = app_state
                        .db_client
                        .update_categoy(
                            body.name, 
                            body.description, 
                            user.user.id, 
                            body.category_id
                        ).await
                        .map_err(|e| {
                            HttpError::server_error(e.to_string())
                        })?.unwrap_or_default();
    Ok(
        (
            StatusCode::OK,
            Json(
                CategoryResponseDto {
                    status : "success",
                    data : CategoryData {
                        category : FilterCategory::filter_category(&updated_category)
                    }
                }
            )
        )
    )
}

pub async fn delete_category(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Path(category_id) : Path<Uuid>
) ->Result<impl IntoResponse,HttpError> {

                         app_state
                        .db_client
                        .delete_category(user.user.id, category_id)
                        .await
                        .map_err(|e| {
                            HttpError::server_error(e.to_string())
                        })?;

    Ok((
        StatusCode::OK,
        Json(
           Response {
            status : "status",
            message : "Deleted user succesfully".to_string()
           }
        )
    ))
    
}