use axum::{
    http::{header, StatusCode}, response::IntoResponse, routing::{get, post}, Extension, Json, Router 
};
use serde_json::json;
use sqlx::{ Pool, Postgres};
mod db;
mod routes;

#[derive(serde::Serialize)]
pub struct ApiError{
    status_code: u8,
    message:String
}

impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        (self.status_code, [(header::CONTENT_TYPE, "appliction/json")], Json(json!("statusCode": self.status_code, "message": self.message)));
    }
}


#[tokio::main]
async fn main() {
    let url = "postgres://postgres:pass@localhost:5432/team-tokens";

    let pool_res = db::db::create_pool(url).await;

    let pool = match pool_res {
        Ok(pool) => pool,
        Err(e) => {
            println!("connection failed: {}", e);
            return;
        }
    };

    if let Err(e) = db::db::run_migration(&pool).await {
        println!("migration failed: {}", e);
        return;
    };



    let router = create_router(pool.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    println!("connection established");
}


fn create_router(pool:Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(routes::healthz::get))
        .route("/v1", post(routes::team::post))
        .layer(Extension(pool))
}


