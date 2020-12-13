use sqlx;
use serde::{Deserialize,Serialize};
use actix_web::{web, Scope, HttpResponse, Responder};
use crate::response::ResBody;

pub const WRITER_TAGS_MODEL: &'static str = r#"
create table if not exists writer_tags (
    id integer primary key asc,
    name text not null,
    title text null,
    create_time text null,
    remark text null
);
"#;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Tag {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub create_time: Option<String>,
    pub remark: Option<String>,
}

pub fn service() -> Scope {
    web::scope("/tags")
    .route("", web::post().to(post_tag))
    .route("", web::get().to(get_tags))
    .route("/{id}", web::get().to(get_tag))
    .route("/{id}", web::delete().to(delete_tag))
    .route("/{id}", web::put().to(put_tag))
}

async fn post_tag() -> impl Responder {
    HttpResponse::Ok().json(ResBody::<String>::default())
}

async fn get_tags() -> impl Responder {
    HttpResponse::Ok().json(ResBody::<String>::default())
}

async fn get_tag() -> impl Responder {
    HttpResponse::Ok().json(ResBody::<String>::default())
}

async fn delete_tag() -> impl Responder {
    HttpResponse::Ok().json(ResBody::<String>::default())
}

async fn put_tag() -> impl Responder {
    HttpResponse::Ok().json(ResBody::<String>::default())
}
