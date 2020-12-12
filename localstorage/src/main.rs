use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqlitePool, Sqlite};
use actix_web::{App, HttpServer};
use actix_rt;

mod service;

const DB_NAME: &'static str = "pims.db";
const SERVER_ADDR: &'static str = "127.0.0.1:8002";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ok = Sqlite::database_exists(DB_NAME).await.unwrap();
    if !ok {
        Sqlite::create_database(DB_NAME).await.unwrap();
    }
    let pool = SqlitePool::connect(DB_NAME).await.unwrap();
    sqlx::query(service::WRITER_MODELS).execute(&pool).await.unwrap();

    HttpServer::new(move||App::new()
        .data(pool.clone())
        .configure(service::service_configure)
    )
    .bind(SERVER_ADDR)?
    .run()
    .await
}
