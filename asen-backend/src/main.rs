use color_eyre::eyre::Result;
use actix_files::Files;
#[allow(unused_imports)]
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};


// Simple server for static file
#[allow(unused_must_use)]
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








