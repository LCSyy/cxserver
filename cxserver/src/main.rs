use actix_web::{HttpServer,App,web};
use actix_rt;

const HTTP_SERVER_ADDR: &'static str = "127.0.0.1:8001";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().route("/api", web::get().to(api))
    )
    .bind(HTTP_SERVER_ADDR)?
    .run()
    .await
}

async fn api() -> String {
    String::from("Hello CxServer !")
}
