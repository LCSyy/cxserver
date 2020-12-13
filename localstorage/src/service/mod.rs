use actix_web::web;

mod writer;
pub use writer::WRITER_BLOGS_MODEL;
pub use writer::WRITER_TAGS_MODEL;

pub fn service_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .service(writer::writer_service())
    );
}
