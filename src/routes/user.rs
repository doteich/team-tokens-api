use std::sync::Arc;

use crate::utils::password;
use crate::utils::password::validate_password;
use crate::utils::password::verify_password;
use crate::utils::token::JWT;
use crate::ApiError;
use axum::extract::State;
use axum::http::Response;
use axum::{Extension, Json};
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserRequest {
    #[validate(length(min = 5, max = 127))]
    name: String,
    password: String,
    #[validate(email)]
    email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct Token {
    token: String,
    validity: i16,
}

pub async fn put(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(body): Json<UserRequest>,
) -> Result<(), ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError {
                client_message: "invalid input".to_string(),
                error_message: e.to_string(),
                status_code: 400,
            });
        }
    }

    let pw = body.password;

    let is_valid = validate_password(pw.clone()).map_err(|e| ApiError {
        client_message: "unable to validate password".to_string(),
        error_message: e.to_string(),
        status_code: 500,
    })?;

    if !is_valid {
        return Err(ApiError {
            client_message: "password does not fullfill the password requirements".to_string(),
            error_message: "invalid password".to_string(),
            status_code: 400,
        });
    }

    let hash_res = password::hash_password(pw.clone());

    let pw_hash = match hash_res {
        Ok(h) => h,
        Err(e) => {
            let x = ApiError {
                client_message: "Password is invalid".to_string(),
                error_message: e.to_string(),
                status_code: 500,
            };
            return Err(x);
        }
    };

    let dt = Utc::now();

    let res = sqlx::query(
        "INSERT INTO users (name, password, email, created_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(body.name)
    .bind(pw_hash)
    .bind(body.email)
    .bind(dt)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError {
                client_message: "unable to process request".to_string(),
                error_message: e.to_string(),
                status_code: 500,
            });
        }
    }

    

    return Ok(());
}


pub async fn login(
    Extension(pool): Extension<Pool<Postgres>>,
    State(jwt): State<Arc<JWT>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<Token>, ApiError> {
    let user_result: Result<(String, String, String), sqlx::Error> =
        sqlx::query_as("SELECT name, password, email FROM users WHERE email=$1")
            .bind(body.email)
            .fetch_one(&pool)
            .await;

    let user_entry = match user_result {
        Ok(row) => row,
        Err(e) => {
            let x = ApiError {
                client_message: "Unable to process request".to_string(),
                error_message: e.to_string(),
                status_code: 500,
            };
            return Err(x);
        }
    };


    let is_valid = match verify_password(body.password, user_entry.1) {
        Ok(b) => b,
        Err(e) => {
            let x = ApiError {
                client_message: "Unable to verify password".to_string(),
                error_message: e.to_string(),
                status_code: 500,
            };
            return Err(x);
        }
    };

    let t = match jwt.create_token() {
        Ok(token) => token,
        Err(e) => {
            let x = ApiError {
                client_message: "Failed to create token".to_string(),
                error_message: e.to_string(),
                status_code: 500,
            };
            return Err(x);
        }
    };

    if !is_valid {
        let x = ApiError {
            client_message: "Password is invalid".to_string(),
            error_message: "user provided invalid password".to_string(),
            status_code: 500,
        };
        return Err(x);
    }

    let res_body = Token {
        token: t,
        validity: 7200,
    };



    Ok(Json(res_body))

    

}
