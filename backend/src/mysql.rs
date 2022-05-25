use anyhow::Result;
use diesel::{MysqlConnection, Connection};
use std::env;
use dotenv::dotenv;
extern crate diesel;

pub fn establish_connection() -> Result<MysqlConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL") ?;
    Ok(MysqlConnection::establish(&database_url)?)
}
