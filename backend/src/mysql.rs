use anyhow::Result;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use std::env;
extern crate diesel;

pub fn establish_connection() -> Result<MysqlConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    Ok(MysqlConnection::establish(&database_url)?)
}
