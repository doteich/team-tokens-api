use axum::{
    Extension, Json,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Deserialize)]
pub struct TeamRequest {
    name: String,
    owner: String,
}


pub async fn post(Extension(pool): Extension<Pool<Postgres>>, Json(body): Json<TeamRequest>) {
    let dt = Utc::now();

    sqlx::query("INSERT INTO teams (owner, name, created_at) VALUES ($1, $2, $3)")
        .bind(body.owner)
        .bind(body.name)
        .bind(dt)
        .execute(&pool)
        .await
        .unwrap();
}
