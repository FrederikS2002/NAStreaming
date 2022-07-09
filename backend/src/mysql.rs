use anyhow::Result;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use std::env;

use crate::errors::ApiError;
extern crate diesel;

pub fn establish_connection() -> Result<MysqlConnection, ApiError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(ApiError::db_error)?;
    Ok(MysqlConnection::establish(&database_url).map_err(ApiError::db_error)?)
}
