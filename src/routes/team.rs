use crate::ApiError;
use axum::{Extension, Json};
use chrono::Utc;
use http::header::HeaderMap;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Deserialize)]
pub struct TeamRequest {
    name: String,
    owner: String,
}
#[axum::debug_handler]
pub async fn put(
    Extension(pool): Extension<Pool<Postgres>>,
    headers: HeaderMap,
    Json(body): Json<TeamRequest>,
) -> Result<(), ApiError> {
    let auth = headers.get("Authorization").unwrap().to_str().unwrap();

    
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
