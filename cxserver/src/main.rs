use std::env;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;
use actix_web::{HttpServer,App,web};
use actix_rt;
use dotenv;

mod service;
use service::SessionManager;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(dotenv::var("PG_MAX_CONNECTION").unwrap().parse().unwrap())
        .connect(&env::var("PG_DATABASE_URL").unwrap())
        .await.unwrap();

    HttpServer::new(move || App::new()
        .data(pool.clone())
        .data(Mutex::new(SessionManager::new()))
        .configure(service::service_config)
        .route("/api", web::get().to(api))
    )
    .bind(dotenv::var("PG_SERVER_ADDR").unwrap())?
    .run()
    .await
}

async fn api(pool: web::Data<sqlx::PgPool>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*pool.into_inner()).await.unwrap();

    row.0.to_string()
}
