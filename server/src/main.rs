use axum::{response::Html, routing::{get, post}, Router};
use db;
mod routes;

#[tokio::main]
async fn main() {
    let url = "postgres://postgres:pass@localhost:5432/team-tokens";

    let pool_res = db::create_pool(url).await;

    let pool = match pool_res {
        Ok(pool) => pool,
        Err(e) => {
            println!("connection failed: {}", e);
            return;
        }
    };

    if let Err(e) = db::run_migration(&pool).await {
        println!("migration failed: {}", e);
        return;
    };

    let router = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    println!("connection established");
}

fn create_router() -> Router{
    Router::new()
    .route("/", get(routes::healthz::get))
    .route("/test", post(routes::team::post))
}
