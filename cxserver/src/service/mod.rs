use actix_web::{web};

mod session;

pub fn init(cfg: &mut web::ServiceConfig) {
    session::init(cfg);
}

