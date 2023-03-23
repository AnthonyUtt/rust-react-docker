use actix_web::{web, App, HttpServer};
use actix_web::{get, post, HttpResponse, Responder};
use std::env;

mod db;
mod routes;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database connection pool
    db::init();

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(format!("0.0.0.0:{}", env::var("PORT").unwrap_or(String::from("8080"))))?
    .run()
    .await
}
