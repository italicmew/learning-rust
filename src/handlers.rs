use actix_web::{Responder, get, post, delete, HttpResponse};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{id}")]
async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/users")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[delete("/users/{id}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}