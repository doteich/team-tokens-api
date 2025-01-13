use axum::{Extension, Json};
use chrono::Utc;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;

use crate::ApiError;

#[derive(Deserialize)]
pub struct TeamRequest {
    name: String,
    owner: String,
}

pub async fn post(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(body): Json<TeamRequest>,
) -> Result<(), ApiError> {
    let dt = Utc::now();

    let res = sqlx::query("INSERT INTO teams (owner, name, created_at) VALUES ($1, $2, $3)")
        .bind(body.owner)
        .bind(body.name)
        .bind(dt)
        .execute(&pool)
        .await;

    match res {
        Ok(q) => Ok(()),
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
