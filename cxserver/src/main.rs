use std::env;
use sqlx::postgres::PgPoolOptions;
use actix_web::{HttpServer,App,web};
use actix_rt;
use dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    HttpServer::new(
        move || App::new().data(pool.clone()).route("/api", web::get().to(api))
    )
    .bind(dotenv::var("SERVER_ADDR").unwrap())?
    .run()
    .await
}

async fn api(pool: web::Data<sqlx::PgPool>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*pool.into_inner()).await.unwrap();

    row.0.to_string()
}
