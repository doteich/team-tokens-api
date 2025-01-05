use sqlx::{
    migrate::MigrateError, postgres::{self, PgPoolOptions}, Error, Pool, Postgres
};

pub async fn create_pool(url: &str) -> Result<Pool<Postgres>, Error> {
    postgres::PgPool::connect(url).await
}

pub async fn run_migration(pool: &Pool<Postgres>) -> Result<(), MigrateError>{
    sqlx::migrate!("./migrations").run(pool).await
}
