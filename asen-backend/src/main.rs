use color_eyre::eyre::Result;
use rocket_contrib::serve::StaticFiles;
#[macro_use]
extern crate rocket;

// Simple server for static file using Rocket
// /home/aryan/Documents/Dev/Rust/asen-combined/asen-front-end/public/
#[rocket::main]
async fn main() -> Result<()> {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(
                "/home/aryan/Documents/Dev/Rust/asen-combined/asen-front-end/public/",
            ),
        )
        .launch()
        .await?;
    Ok(())
}
