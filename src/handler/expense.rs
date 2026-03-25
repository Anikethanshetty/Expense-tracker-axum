use std::{str::FromStr, sync::Arc};

use axum::{Extension, Json, extract::{Path, Query}, http::StatusCode, response::IntoResponse};
use bigdecimal::{BigDecimal, FromPrimitive};
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, database::expense::ExpenseExt, dtos::{Response, expense::{CreateExpenseDto, DeleteExpenseDto, ExpenseData, ExpenseResponseDto, ExpensesData, ExpensesResponseDto, FilterExpense, GetExpensesQuery, UpdateExpenseDto}}, error::{ErrorMessage, HttpError}, middleware::JwtAuthMiddleware};


pub async fn create_expense(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(body) : Json<CreateExpenseDto>
) -> Result<impl IntoResponse,HttpError> {
    body.validate().map_err(|e|{
        HttpError::server_error(e.to_string())
    })?;

    let amount = BigDecimal::from_f64(body.amount).unwrap_or_default();


    let expense = app_state
                            .db_client
                            .create_expense(
                                amount, 
                                body.expenses_date, 
                                user.user.id, 
                                body.category_id
                            ).await
                            .map_err(|e|{
                                HttpError::server_error(e.to_string())
                            })?;
    Ok(
        (
            StatusCode::OK,
            Json(
                ExpenseResponseDto {
                    status : "success",
                    data : ExpenseData {
                        expense : FilterExpense::filter_expense(&expense)
                    }
                }
            )
        )
)
}

pub async fn get_all_expenses(
    Query(params) : Query<GetExpensesQuery>,
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Path(category_id) : Path<Uuid>
) ->Result<impl IntoResponse,HttpError> {

    let expenses = app_state
                            .db_client
                            .get_all_expenses(
                                params.page, 
                                params.limit, 
                                user.user.id, 
                                category_id
                            ).await.map_err(|e| {
                                HttpError::server_error(e.to_string())
                            })?.unwrap_or_default();

        let filtered_expenses : Vec<FilterExpense> = expenses.iter().map(FilterExpense::filter_expense)
                                        .collect();
            
    Ok(
       ( StatusCode::OK,
        Json(
            ExpensesResponseDto {
                status : "success",
                data : {
                    ExpensesData { expense: filtered_expenses }
                }
            }
        ))
    )

}


pub async fn get_expense(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Path(category_id) : Path<Uuid>,
    Path(expense_id) : Path<Uuid>
) -> Result<impl IntoResponse,HttpError>{

    let user = user.user;
    let expense = app_state
                                .db_client
                                .get_expense(
                                    user.id, 
                                    category_id,
                                    expense_id,                                  
                                ).await.map_err(|e| {
                                    HttpError::server_error(e.to_string())
                                }
                            )?.unwrap_or_default();

            Ok(
               ( StatusCode::OK,
                Json (
                    ExpenseResponseDto {
                        status : "success",
                        data : ExpenseData { 
                            expense : FilterExpense::filter_expense(&expense) 
                        }
                    }
                ))
            )
}

pub async fn update_expense(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body): Json<UpdateExpenseDto>
) -> Result<impl IntoResponse,HttpError>
    {

    let amount = body.amount.map(|val|{
            BigDecimal::from_str(&val.to_string()).unwrap()
    }); 

    let updated_expense = app_state
                                        .db_client
                                        .update_expense(
                                            amount, 
                                            body.expense_date, 
                                            user.user.id, 
                                            body.category_id, 
                                            body.expense_id
                                        ).await.map_err(|e|{
                                            HttpError::server_error(e.to_string())
                                        })?.unwrap_or_default();

            Ok(
               (StatusCode::OK,
                Json(
                    ExpenseResponseDto {
                        status : "success",
                        data : ExpenseData { 
                            expense: FilterExpense::filter_expense(&updated_expense) 
                        }
                    }
                ))
            )
}

    pub async fn delete_expense(
        Extension(app_state) : Extension<Arc<AppState>>,
        Extension(user) : Extension<JwtAuthMiddleware>,
        Json(body):Json<DeleteExpenseDto>
) 
    -> Result<impl IntoResponse,HttpError>
    {
        
         let _ = app_state
            .db_client
            .delete_expense(user.user.id, body.category_id, body.expense_id)
            .await
            .map_err(|e|{
                HttpError::server_error(e.to_string());
            });

            Ok(
                (
                    StatusCode::OK,
                   Json( 
                    Response {
                        status : "success",
                        message : "Deleted expense succesfully".to_string() 
                    })
                )
            )
         
}