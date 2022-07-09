use crate::schema::*;
use anyhow::Result;
use serde::Serialize;
use diesel::{prelude::*, MysqlConnection};

#[derive(Queryable, Debug, Serialize)]
pub struct MovieDetails {
    pub id: i32,
    pub uuid: String,
    pub thumb: String,
    pub icon: String,
    pub description: String,
}
pub struct MovieDetailService<'a> {
    pub conn: &'a MysqlConnection,
}

impl<'a> MovieDetailService<'a> {
    pub fn get(&self, uuidc: String) -> Result<Vec<MovieDetails>> {
        use crate::schema::movie_details::dsl::{movie_details, uuid};
        Ok(movie_details
            .filter(uuid.eq(uuidc))
            .limit(1)
            .load::<MovieDetails>(self.conn)?)
    }
}

#[derive(Insertable, Debug)]
#[table_name = "movie_details"]
pub struct NewMovieDetails {
    pub uuid: String,
    pub thumb: String,
    pub icon: String,
    pub description: String,
}
