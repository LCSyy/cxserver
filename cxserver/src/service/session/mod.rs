use actix_web::{web, Responder, HttpResponse};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/session")
        .route("", web::post().to(login))
        .route("", web::delete().to(logout))
    );
}

async fn login() -> impl Responder {
    HttpResponse::Ok().json(format!("{{ session id: {}, user: {} }}", "a","b"))
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().json("{}")
}
