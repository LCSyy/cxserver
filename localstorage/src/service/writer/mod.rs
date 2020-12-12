use actix_web::{web, Scope, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool};
use sqlx::Row;

pub const WRITER_MODELS: &'static str = r#"
create table if not exists writer_blogs (
    id integer primary key asc,
    title text null,
    content text null,
    tags text null,
    status text null,
    create_time text null
);
"#;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct Blog {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>,
    tags: Option<String>, 
    status: Option<String>,
    create_time: Option<String>,
}

#[derive(Default, Serialize)]
struct ResBody<T> where T: Serialize {
    err: Option<String>,
    data: Option<T>,
}

pub fn writer_service() -> Scope {
    web::scope("/writer")
    .service(web::scope("/v1")
        .service(web::scope("/blogs")
            .route("", web::post().to(post_blog))
            .route("", web::get().to(get_blogs))
            .route("/{id}", web::get().to(get_blog))
            .route("/{id}", web::delete().to(delete_blog))
            .route("/{id}", web::put().to(put_blog))
        )
    )
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
    
    let row = sqlx::query("select id, datetime(create_time,'localtime') from writer_blogs where id = last_insert_rowid();").fetch_one(&*pool.clone()).await.unwrap();
    b.id = Some(row.try_get(0).unwrap());
    b.create_time = Some(row.try_get(1).unwrap());

    HttpResponse::Ok().json(ResBody {err: None, data: Some(b) })
}

async fn get_blogs(pool: web::Data<SqlitePool>) -> impl Responder {
    let pool = pool.into_inner();
    let rows = sqlx::query("select id, title, content, tags, status, datetime(create_time,'localtime') from writer_blogs;").fetch_all(&*pool.clone()).await.unwrap();

    let mut res = vec![];
    for row in rows {
        res.push(Blog {
            id: Some(row.try_get(0).unwrap()),
            title: Some(row.try_get(1).unwrap()),
            content: Some(row.try_get(2).unwrap()),
            tags: Some(row.try_get(3).unwrap()),
            status: Some(row.try_get(4).unwrap()),
            create_time: Some(row.try_get(5).unwrap()),
        });
    }

    HttpResponse::Ok().json(ResBody { err: None, data: Some(res) })
}

async fn get_blog(pool: web::Data<SqlitePool>, id: web::Path<i32>) -> impl Responder {
    let pool = pool.into_inner();
    let row = match sqlx::query("select id, title, content, tags, status, datetime(create_time,'localtime') from writer_blogs where id = ?;")
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

    let res = Blog {
        id: Some(row.try_get(0).unwrap()),
        title: Some(row.try_get(1).unwrap()),
        content: Some(row.try_get(2).unwrap()),
        tags: Some(row.try_get(3).unwrap()),
        status: Some(row.try_get(4).unwrap()),
        create_time: Some(row.try_get(5).unwrap()),
    };

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

    let row = sqlx::query("select datetime(create_time,'localtime') from writer_blogs where id = ?;").bind(id).fetch_one(&*pool.clone()).await.unwrap();
    b.create_time = Some(row.try_get(0).unwrap());

    HttpResponse::Ok().json(ResBody{ err: None, data: Some(b) })
}
