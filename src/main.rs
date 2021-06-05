use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use handlers::*;
use mongodb::{options::{ClientOptions, ResolverConfig}, Client};
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let client_options = ClientOptions::parse_with_resolver_config("mongodb://localhost:27017", ResolverConfig::cloudflare()).await.unwrap();
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/users")
                .route("", web::get().to(handlers::get_users))
                .app_data(client.clone())
            )
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}