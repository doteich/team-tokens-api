use crate::utils::password;
use crate::utils::token::JWT;
use crate::ApiError;
use axum::extract::State;
use axum::{Extension, Json};
use chrono::Utc;
use http::header::HeaderMap;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::num::ParseIntError;
use std::sync::Arc;

use super::user;

#[derive(Deserialize)]
pub struct TeamRequest {
    name: String,
    owner: String,

    password: String,
}
#[axum::debug_handler]
pub async fn put(
    Extension(pool): Extension<Pool<Postgres>>,

    headers: HeaderMap,
    State(jwt): State<Arc<JWT>>,
    Json(body): Json<TeamRequest>,
) -> Result<(), ApiError> {
    let auth_header = headers.get("Authorization");

    let auth = match auth_header {
        Some(a) => a,
        None => {
            return Err(ApiError {
                error_message: "no auth header".to_string(),
                client_message: "no auth header was provided".to_string(),
                status_code: 401,
            })
        }
    };

    let token = auth.to_str().map_err(|e| ApiError {
        error_message: e.to_string(),
        client_message: "unable to parse token".to_string(),
        status_code: 401,
    })?;

    let user_id = jwt.verify_token(token).map_err(|e| ApiError {
        error_message: e.to_string(),
        client_message: "unable to verify token".to_string(),
        status_code: 401,
    })?;

    if user_id.is_empty() {
        return Err(ApiError {
            error_message: "invalid token".to_string(),
            client_message: "provided an invalid token".to_string(),
            status_code: 401,
        });
    }

    if !password::validate_password(body.password.clone()).map_err(|e| ApiError {
        error_message: e.to_string(),
        client_message: "invalid password provided".to_string(),
        status_code: 500,
    })? {
        return Err(ApiError {
            error_message: "invalid password".to_string(),
            client_message: "password must contain at least one uppercase letter, one number, and one special character".to_string(),
            status_code: 400,
        });
    }

    let i: i64 = user_id.parse().map_err(|e: ParseIntError| ApiError {
        error_message: e.to_string(),
        client_message: "unable to parse user id".to_string(),
        status_code: 500,
    })?;

    let pw = password::hash_password(body.password.clone()).map_err(|e| ApiError {
        error_message: e.to_string(),
        client_message: "invalid password provided".to_string(),
        status_code: 500,
    })?;

    let dt = Utc::now();

    let res = sqlx::query("INSERT INTO teams (owner_name, owner_id, name, created_at, password) VALUES ($1, $2, $3, $4, $5)")
        .bind(body.owner)
        .bind(i)
        .bind(body.name)
        .bind(dt)
        .bind(pw)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(ApiError {
                error_message: e.to_string(),
                client_message: "unable to store data".to_string(),
                status_code: 500,
            })
        }
    }
}

pub async fn get() {}
