use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, Responder, HttpResponse};

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/session")
        .route("",web::post().to(login))
        .route("",web::delete().to(logout))
    );
}

async fn login(scs: web::Data<Mutex<SessionManager>>) -> impl Responder {
    let s = Session{
        expire_time: 0,
        id: 213542353,
        name: String::from("lcs"),
        password: String::from("fsdfatfaq351rc1c42"),
        session_id: String::from("zz11"),
    };

    scs.lock().unwrap().add_session("1", s.clone());

    HttpResponse::Ok().json(format!("{{ session id: {}, user: {} }}", s.session_id, s.name))
}

async fn logout(scs: web::Data<Mutex<SessionManager>>) -> impl Responder {
    match scs.lock().unwrap().remove_session("1") {
        Some(s) => println!("delete session: {}", s.id),
        None => println!("has no such session"),
    }

    HttpResponse::Ok().json("{}")
}

#[derive(Clone)]
pub struct Session {
    pub expire_time: u64,
    pub id: u64,
    pub name: String,  
    pub password: String,
    pub session_id: String,
}

pub struct SessionManager {
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
        }
    }

    pub fn add_session(&mut self, k: &str, s: Session) {
        self.sessions.insert(k.to_owned(), s);
    }

    pub fn session(&self, k: &str) -> Option<&Session> {
        self.sessions.get(k)
    }

    pub fn remove_session(&mut self, k: &str) -> Option<Session> {
        self.sessions.remove(k)
    }
}
