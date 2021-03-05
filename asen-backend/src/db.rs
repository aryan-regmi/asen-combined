use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use color_eyre::eyre::Result;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must beb set");
    
    Ok(PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url)))
}
