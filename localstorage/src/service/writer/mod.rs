mod blogs;
mod tags;

use actix_web::{web, Scope};

pub use blogs::WRITER_BLOGS_MODEL;
pub use tags::WRITER_TAGS_MODEL;

pub fn writer_service() -> Scope {
    web::scope("/writer")
    .service(web::scope("/v1")
        .service(blogs::service())
        .service(tags::service())
    )
}
