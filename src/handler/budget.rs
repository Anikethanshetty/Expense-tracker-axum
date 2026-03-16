use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use bigdecimal::{BigDecimal, FromPrimitive};
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, database::budget::BudgetExt, dtos::{Response, budget::{BudgetData, BudgetResponse, CreateBudgetDto, FilterBudget, UpdateBudgetDto}}, error::HttpError, middleware::JwtAuthMiddleware};


pub async fn create_budget(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(body) : Json<CreateBudgetDto>
)->Result<impl IntoResponse,HttpError> {

    body.validate().map_err(|e| {
        HttpError::bad_request(e.to_string())
    })?;

    let amount = BigDecimal::from_f64(body.amount).unwrap_or_default();

    let budget = app_state
                        .db_client
                        .create_budget(user.user.id, body.category_id, amount)
                        .await
                        .map_err(|e|{
                            HttpError::server_error(e.to_string())
                        })?;
    Ok(
        (
            StatusCode::OK,
            Json(
                BudgetResponse{
                    status : "success",
                    data : BudgetData {
                        budget : FilterBudget::filter_budget(&budget)
                    }
                }
            )
        )
    )
}

pub async fn get_budget(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Path(category_id):Path<Uuid>
) ->Result<impl IntoResponse,HttpError> {

    let budget = app_state
                        .db_client
                        .get_budget(user.user.id, category_id)
                        .await.map_err(|e| {
                            HttpError::server_error(e.to_string())
                        })?.unwrap_or_default();
    Ok((
        StatusCode::OK,
        Json(
            BudgetResponse {
                status : "success",
                data : BudgetData {
                    budget : FilterBudget::filter_budget(&budget)
                }
            }
        )
    ))
}

pub async fn update_budget(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(body) : Json<UpdateBudgetDto>
) -> Result<impl IntoResponse,HttpError> {
    
    let amount = BigDecimal::from_f64(body.amount).unwrap_or_default();

    let updated_budget = app_state
                            .db_client
                            .update_budget(user.user.id, 
                                body.category_id, 
                                body.budget_id, 
                                amount
                            ).await.map_err(|e|{
                                HttpError::server_error(e.to_string())
                            })?.unwrap_or_default();
    Ok((
        StatusCode::OK,
        Json(
            BudgetResponse {
                status : "success",
                data : BudgetData {
                    budget : FilterBudget::filter_budget(&updated_budget)
                }                
            }
        )
    ))
}

pub async fn delete_budget(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Path(category_id): Path<Uuid>,
    Path(budget_id): Path<Uuid>
) -> Result<impl IntoResponse,HttpError> {

                app_state
                        .db_client
                        .delete_budget(
                            user.user.id, 
                            category_id, 
                            budget_id
                        ).await.map_err(|e|{
                            HttpError::server_error(e.to_string())
                        })?;

    Ok((
        StatusCode::OK,
        Json(
            Response {
                status : "success",
                message : "Budget Deleted Succesfully".to_string()
            }
        )
    ))
}