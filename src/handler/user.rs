use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{AppState, database::user::UserExt, dtos::{Response, user::{DeleteUserDto, FilterUserDto, UpdateUserDto, UserData, UserResponse}}, error::{ErrorMessage, HttpError}, middleware::JwtAuthMiddleware};


pub async fn update_user(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(body): Json<UpdateUserDto>
) ->Result<impl IntoResponse,HttpError>{

     body.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;

    let user = user.user;

    let username = body.username;
    let email = body.email;

    let updated_user = app_state
                    .db_client
                    .update_user(user.id, username, email)
                    .await
                    .map_err(|_|{
                        HttpError::server_error(ErrorMessage::ServerError.to_string())
                    })?
                    .ok_or(HttpError::server_error(ErrorMessage::ServerError.to_string()))?;

    
    
    Ok((
        StatusCode::OK,
        Json(
            UserResponse {
                status: "success",
                data: UserData {
                    user : FilterUserDto::filter_user(&updated_user, None, None, None)
                }
            }
        )
    ))
}

pub async fn delete_user(
    Extension(app_state) : Extension<Arc<AppState>>,
    Json(body) : Json<DeleteUserDto>
) ->Result<impl IntoResponse,HttpError> {
         body.validate().map_err(|e| {
        HttpError::bad_request(e.to_string())
    })?;

    let deleted = app_state
            .db_client.delete_user(body.email).await.map_err(|_| {
                HttpError::server_error(ErrorMessage::ServerError.to_string())
            })?;


    Ok((
      StatusCode::OK,
      Json(
        Response {
            status:"success",
            message : "User deleted Successfully".to_string()
        }
      )  
    ))
}