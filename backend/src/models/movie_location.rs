use crate::schema::*;
use anyhow::Result;
use diesel::{prelude::*, MysqlConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct MovieFileLoation {
    id: i32,
    pub uuid: String,
    pub movie: String,
    title: String,
    pub epi: i32,
    pub filename: String,
    description: String,
    thumb: String,
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

    pub fn show_movie_loc_single(
        &self,
        mv: String,
        uuidl: String,
    ) -> Result<Vec<MovieFileLoation>> {
        use crate::schema::movie_filelocations::dsl::{movie, movie_filelocations, uuid};
        Ok(movie_filelocations
            .filter(movie.eq(mv))
            .filter(uuid.eq(uuidl))
            .load::<MovieFileLoation>(self.conn)?)
    }

    pub fn show_next_movie(&self, mv: String, prev: i32) -> Result<Vec<MovieFileLoation>> {
        use crate::schema::movie_filelocations::dsl::{epi, movie, movie_filelocations};
        Ok(movie_filelocations
            .filter(movie.eq(mv))
            .filter(epi.eq(prev + 1))
            .load::<MovieFileLoation>(self.conn)?)
    }

    pub fn show_movie_loc_multi(&self, mv: String) -> Result<Vec<MovieFileLoation>> {
        use crate::schema::movie_filelocations::dsl::{movie, movie_filelocations};
        Ok(movie_filelocations
            .filter(movie.eq(mv))
            .load::<MovieFileLoation>(self.conn)?)
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
    pub title: String,
    pub filename: String,
    pub description: String,
    pub thumb: String,
}
