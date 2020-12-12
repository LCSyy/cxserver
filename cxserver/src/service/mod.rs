use actix_web::{web};

mod session;
pub use session::SessionManager;

pub fn service_config(cfg: &mut web::ServiceConfig) {
    session::service_config(cfg);
}

