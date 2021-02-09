use actix_web::{HttpResponse, Responder, web};

pub fn init(c: &mut web::ServiceConfig) {
    c.service(web::scope("/user")
        .route("", web::get().to(get))
        .route("/me", web::get().to(me))
    );
}

async fn get() -> impl Responder {
    HttpResponse::Ok().body("")
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("")
}
