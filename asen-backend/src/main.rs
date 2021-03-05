mod db;
//use db::establish_connection;

use rocket_contrib::serve::StaticFiles;

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

#[get("/test")]
async fn test_db() -> Result<String, String> {
    db::establish_connection().unwrap();
    Ok(format!("Connected to database!"))
}

// Simple server for static file using Rocket
// /home/aryan/Documents/Dev/Rust/asen-combined/asen-front-end/public/
#[rocket::main]
async fn main() -> color_eyre::Result<()> {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(
                "/home/aryan/Documents/Dev/Rust/asen-combined/asen-front-end/public/",
            ),
        )
        .mount(
            "/",
            routes![test_db]
        )
        .launch()
        .await?;
    Ok(())
}
