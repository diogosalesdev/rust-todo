pub mod model;
pub mod response;
pub mod handler;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

use crate::handler::health_checker_handler;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    env_logger::init();

    println!("🚀 Server started successfully!");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)  
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
