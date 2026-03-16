use std::sync::Arc;

use axum::{Extension, Json, http::{HeaderMap, StatusCode, header}, response::IntoResponse};
use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{AppState, database::{auth::AuthExt, user::UserExt}, 
    dtos::{Response, auth::{LoginUserDto, RegisterUserDto}, user::{FilterUserDto, UserData, UserResponse}}, error::{ErrorMessage, HttpError}, utils::{password, token}};


pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body):Json<RegisterUserDto>
)->Result<impl IntoResponse,HttpError> {

        body.validate().map_err(|e| {
            HttpError::bad_request(e.to_string())
        })?;

        let hash_password = password::hash(
                                    &body.password)
                                    .map_err(|e|
                                        HttpError::bad_request(
                                            e.to_string()
                                        ))?;

      let result = app_state
                .db_client.create_user(
                 &body.username, 
                 &body.email, 
                 &hash_password
            ).await;


            match result {
                Ok(val) => {
                    Ok((
                        StatusCode::CREATED,
                        Json(
                            Response {
                                message:"User Regsitered successfully".to_string(),
                                status: "success"
                            }
                        )
                    ))
                }
                Err(sqlx::Error::Database(db_error)) => {
                        if db_error.is_unique_violation() {
                            Err(HttpError::unique_constraint_violation(db_error.to_string()))
                        }else {
                            Err(HttpError::server_error(ErrorMessage::ServerError.to_string()))
                        }

                }
                Err(e) => {
                    Err(HttpError::server_error(e.to_string()))
                }
            }
        
}

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {

    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .db_client
        .login(&body.email)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = user.ok_or(
        HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
    )?;

    let password_valid = password::compare(
        &body.password,
        &user.password
    ).map_err(|e| HttpError::server_error(e.to_string()))?;

    if !password_valid {
        return Err(HttpError::unauthorized(
            ErrorMessage::WrongCredentials.to_string(),
        ));
    }

    let user_id = user.id.to_string();

    let token = token::create_token(
        &user_id, app_state.env.jwt_secret.as_bytes(),
         app_state.env.jwt_maxage)
         .map_err(|e|{
            HttpError::server_error(e.to_string())
         })?;

        let mut headers = HeaderMap::new();

        let cookie_duration = time::Duration::minutes(
            app_state.env.jwt_maxage * 60 
        );

        let cookie = Cookie::build(("token",token))
                        .path("/")
                        .max_age(cookie_duration)
                        .build();

        headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    return 
        Ok((
            StatusCode::OK,
            Json(
                UserResponse {
                    status: "success",
                    data:UserData { 
                        user: FilterUserDto::filter_user(&user, 
                            None, None, None)
                    }
                }
            )
        ).into_response().headers_mut().extend(headers));
}