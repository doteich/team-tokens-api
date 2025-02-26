use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    routing::{get, post, put},
    Extension, Json, Router,
};
use env_logger;
use log::{error, info, warn};
use routes::user;
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::fs;
use std::sync::Arc;
use utils::token::{init_jwt, JWT};
mod db;
mod routes;
mod utils;

#[derive(serde::Serialize)]
pub struct ApiError {
    error_message: String,
    status_code: u16,
    client_message: String,
}

#[derive(serde::Deserialize)]
struct Config {
    db: DbConfig,
}
#[derive(serde::Deserialize)]
struct DbConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    db: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        warn!(
            "request failed: {} - client message: {}",
            self.error_message, self.client_message
        );

        let status_code =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let error_body = json!({
            "message": self.client_message
        });

        (status_code, Json(error_body)).into_response()
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let conf_res = read_conf();

    let conf = match conf_res {
        Ok(c) => c,
        Err(e) => {
            error!("invalid config file: {}", e);
            return;
        }
    };

    let con_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        conf.db.user, conf.db.password, conf.db.host, conf.db.port, conf.db.db
    );

    //let url = "postgres://postgres:pass@localhost:5432/team-tokens";

    let pool_res = db::db::create_pool(&con_str[..]).await;

    let pool = match pool_res {
        Ok(pool) => pool,
        Err(e) => {
            error!("connection failed: {}", e);
            return;
        }
    };

    if let Err(e) = db::db::run_migration(&pool).await {
        error!("migration failed: {}", e);
        return;
    };

    let jwt_instance = Arc::new(init_jwt());


    let router = create_router(pool.clone(), jwt_instance);

    info!("server now acception connection on port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    info!("server now acception connection on port 3000")
}

fn read_conf() -> Result<Config, Box<dyn std::error::Error>> {
    let res = fs::read_to_string("./config/config.json")?;

    let config = serde_json::from_str(&res[..])?;

    Ok(config)
}

fn create_router(pool: Pool<Postgres>, jwt: Arc<JWT>) -> Router {
    Router::new()
        .route("/", get(routes::healthz::get))
        .route("/v1/teams", put(routes::team::put))
        .route("/v1/user", put(routes::user::put))
        .route("/v1/user/login", post(user::login))
        .layer(Extension(pool))
        .with_state(jwt) // I want to inject the jwt here 
}
