use std::env;
use sqlx::postgres::{PgPoolOptions,PgPool};
use actix_web::{HttpServer,App};
use actix_rt;
use dotenv;

mod service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(dotenv::var("PG_MAX_CONNECTION").unwrap().parse().unwrap())
        .connect(&env::var("PG_DATABASE_URL").unwrap())
        .await.unwrap();

    HttpServer::new(move || App::new()
        .data(PgPool::clone(&pool))
        .configure(service::init)
    )
    .bind(dotenv::var("PG_SERVER_ADDR").unwrap())?
    .run()
    .await
}
