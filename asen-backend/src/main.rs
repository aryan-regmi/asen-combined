use eyre::{Result};
use thiserror::Error;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[derive(Error, Debug)]
#[from(std::io::Error)]
pub enum MainError {
    #[error("Invalid File Yo")]
    InvalidFi()
}

// Simple server for static file
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                Files::new(
                    "/", "/home/aryan/Documents/Dev/Rust/asen-combined/asen-front-end/public/")
                .index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await;
    
    Ok(())
}








