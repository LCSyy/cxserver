use actix_web::{web, Scope, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool};
use sqlx::{Row, FromRow};
use crate::response::ResBody;

pub const WRITER_BLOGS_MODEL: &'static str = r#"
create table if not exists writer_blogs (
    id integer primary key asc,
    title text null,
    content text null,
    tags text null,
    status text null,
    create_time text null
);
"#;

#[derive(Default, FromRow, Deserialize, Serialize)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>, 
    pub status: Option<String>,
    pub create_time: Option<String>,
}

pub fn service() -> Scope {
    web::scope("/blogs")
    .route("", web::post().to(post_blog))
    .route("", web::get().to(get_blogs))
    .route("/{id}", web::get().to(get_blog))
    .route("/{id}", web::delete().to(delete_blog))
    .route("/{id}", web::put().to(put_blog))
}

async fn post_blog(pool: web::Data<SqlitePool>, blog: web::Json<Blog>) -> impl Responder {
    let pool = pool.into_inner();
    let mut b = blog.into_inner();

    if let Err(e) = sqlx::query("insert into writer_blogs (title, content, tags, status, create_time) values (?,?,?,?,datetime('now'));") // save as utc datetime
        .bind(b.title.clone())
        .bind(b.content.clone())
        .bind(b.tags.clone())
        .bind(b.status.clone())
        .execute(&*pool.clone())
        .await
    {
        return HttpResponse::InternalServerError()
            .json(ResBody::<String> {
                err: Some(e.to_string()),
                data: None,
            })
    };
    
    let row = sqlx::query("select * from writer_blogs where id = last_insert_rowid();").fetch_one(&*pool.clone()).await.unwrap();
    b = FromRow::from_row(&row).unwrap();

    HttpResponse::Ok().json(ResBody {err: None, data: Some(b) })
}

async fn get_blogs(pool: web::Data<SqlitePool>) -> impl Responder {
    let pool = pool.into_inner();
    let rows = sqlx::query("select id, title, content, tags, status, datetime(create_time,'localtime') as create_time from writer_blogs;")
        .fetch_all(&*pool.clone()).await.unwrap();

    let mut res: Vec<Blog> = vec![];
    for row in rows {
        res.push(FromRow::from_row(&row).unwrap());
    }

    HttpResponse::Ok().json(ResBody { err: None, data: Some(res) })
}

async fn get_blog(pool: web::Data<SqlitePool>, id: web::Path<i32>) -> impl Responder {
    let pool = pool.into_inner();
    let row = match sqlx::query("select id, title, content, tags, status, datetime(create_time,'localtime') as create_time from writer_blogs where id = ?;")
        .bind(id.into_inner())
        .fetch_one(&*pool.clone())
        .await
    {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest()
            .json(ResBody::<String> {
                err: Some(e.to_string()),
                data: None,
            }),
    };

    let res: Blog = FromRow::from_row(&row).unwrap();

    HttpResponse::Ok().json(ResBody{ err: None, data: Some(res) })
}

async fn delete_blog(pool: web::Data<SqlitePool>, id: web::Path<i32>) -> impl Responder {
    let pool = pool.into_inner();

    if let Err(e) = sqlx::query("delete from writer_blogs where id = ?;")
        .bind(id.into_inner())
        .execute(&*pool.clone())
        .await
    {
        HttpResponse::InternalServerError()
        .json(ResBody::<String> {
            err: Some(e.to_string()),
            data: None
        })
    } else {
        HttpResponse::Ok().json(ResBody::<String>::default())
    }
}

async fn put_blog(pool: web::Data<SqlitePool>, id: web::Path<i32>, blog: web::Json<Blog>) -> impl Responder {
    let pool = pool.into_inner();
    let id = id.into_inner();
    let mut b = blog.into_inner();
    b.id = Some(id);
    
    if let Err(e) = sqlx::query("update writer_blogs set title = ?, content = ?, tags = ?, status = ? where id = ?;") // save utc datetime
        .bind(b.title.clone())
        .bind(b.content.clone())
        .bind(b.tags.clone())
        .bind(b.status.clone())
        .bind(id)
        .execute(&*pool.clone()).await
    {
        return HttpResponse::InternalServerError().json(ResBody::<String> {err: Some(e.to_string()), data: None });
    }

    let row = sqlx::query("select id, title, content, tags, status, datetime(create_time,'localtime') as create_time from writer_blogs where id = ?;")
        .bind(id).fetch_one(&*pool.clone()).await.unwrap();
    b = FromRow::from_row(&row).unwrap();

    HttpResponse::Ok().json(ResBody{ err: None, data: Some(b) })
}
