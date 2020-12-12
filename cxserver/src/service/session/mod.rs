use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, Responder, HttpResponse};

pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/session")
        .route("",web::post().to(login))
        .route("",web::delete().to(logout))
    );
}

async fn login(scs: web::Data<Mutex<SessionManager>>) -> impl Responder {
    let s = Session{
        id: 213542353,
        name: String::from("lcs"),
        password: String::from("fsdfatfaq351rc1c42"),
        session_id: String::from("zz11"),
    };

    scs.into_inner().lock().unwrap().add_session("1", s.clone());

    HttpResponse::Ok().body(format!("Login with {{ session id: {}, user: {} }}", s.session_id, s.name))
}

async fn logout(scs: web::Data<Mutex<SessionManager>>) -> impl Responder {
    match scs.into_inner().lock().unwrap().session("zz11") {
        Some(s) => HttpResponse::Ok().body(format!("logut {}", s.session_id)),
        None => HttpResponse::Ok().body("None")
    }
}

#[derive(Clone)]
pub struct Session {
    pub id: u64,
    pub name: String,  
    pub password: String,
    pub session_id: String,
}

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Arc::new(Mutex::new(HashMap::new()),)
        }
    }

    pub fn add_session(&mut self, k: &str, s: Session) {
        self.sessions.lock().unwrap().insert(k.to_owned(), s);
    }

    pub fn session(&self, k: &str) -> Option<Session> {
        match self.sessions.lock().unwrap().get(k) {
            Some(s) => Some(s.clone()),
            _ => None,
        }
    }
}
