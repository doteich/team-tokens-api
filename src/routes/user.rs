use crate::utils::password;
use crate::utils::password::validate_password;
use crate::utils::password::verify_password;
use crate::ApiError;
use axum::{Extension, Json};
use chrono::Utc;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Deserialize)]
pub struct UserRequest {
    name: String,
    password: String,
    email: String,
}

pub async fn put(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(body): Json<UserRequest>,
) -> Result<(), ApiError> {
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

    //sqlx::query("INSERT INTO")


    return Ok(());
}
