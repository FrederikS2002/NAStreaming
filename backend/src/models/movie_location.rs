use crate::schema::*;
use anyhow::Result;
use diesel::{prelude::*, MysqlConnection};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct MovieFileLoation {
    id: i32,
    uuid: String,
    movie: String,
    epi: i32,
    name: String,
    filename: String,
}

pub struct MovieFileLoationService<'a> {
    pub conn: &'a MysqlConnection,
}

impl<'a> MovieFileLoationService<'a> {
    // pub fn all(&self) -> Result<Vec<Movie>> {
    //     use diesel::prelude::*;
    //     use super::schema::movies::dsl::movies;
    //     Ok(movies.load::<Movie>(self.conn)?)
    // }

    pub fn show(&self, id: i32) -> Result<MovieFileLoation> {
        use crate::schema::movie_filelocations::dsl::movie_filelocations;
        let movie: MovieFileLoation = movie_filelocations.find(id).first(self.conn)?;
        Ok(movie)
    }

    pub fn delete(&self, sql_index: i32) -> Result<()> {
        use crate::schema::movie_filelocations::dsl::*;
        diesel::delete(movie_filelocations.find(sql_index)).execute(self.conn)?;
        Ok(())
    }

    pub fn add(&self, req: NewMovieFileLoation) -> Result<MovieFileLoation> {
        use crate::schema::movie_filelocations::table;

        let sq = diesel::insert_into(table).values(req).execute(self.conn)?;
        return self.show(sq as i32);
    }
}

#[derive(Insertable, Debug)]
#[table_name = "movie_filelocations"]
pub struct NewMovieFileLoation {
    pub uuid: String,
    pub movie: String,
    pub epi: i32,
    pub name: String,
    pub filename: String,
}
