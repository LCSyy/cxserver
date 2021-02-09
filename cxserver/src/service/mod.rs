use actix_web::{web};

mod session;
mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    session::init(cfg);
    user::init(cfg);
}

