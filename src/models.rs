use actix_web::{HttpResponse, Responder, get, post, web};
use std::sync::Mutex;

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>,
}

pub struct AppState {
    pub app_name: String,
}

#[get("/")]
pub async fn hello(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

pub async fn tambah(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Counter= {counter}")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn masuk(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hey There {app_name}"))
}

pub async fn index() -> impl Responder {
    "Hei Ho"
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn users(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    format!("Welcome {}, user_id {}!", friend, user_id)
}