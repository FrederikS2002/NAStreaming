use super::schema::*;
use diesel::MysqlConnection;
use anyhow::Result;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Movie {
    id: i32,
    uuid: String,
    type_: String,
    titles: String,
    categories: String,
    age_restriction: i32,
    img_src: String,

}

pub struct MovieService<'a> {
    pub conn: &'a MysqlConnection
}

impl<'a> MovieService<'a> {
    pub fn all(&self) -> Result<Vec<Movie>> {
        use diesel::prelude::*;
        use super::schema::movies::dsl::movies;
        Ok(movies.load::<Movie>(self.conn)?)
    }

    pub fn show(&self, id:i32) -> Result<Movie> {
        use diesel::prelude::*;
        use super::schema::movies::dsl::movies;
        let movie: Movie = movies.find(id).first(self.conn)?;
        Ok(movie)
    }

    pub fn show_page(&self, search: &str,page:i64, pagesize: i64) -> Result<Vec<Movie>> {
        use diesel::prelude::*;
        use super::schema::movies::dsl::*;
        Ok(movies.filter(titles.like(format!("%{}%", search))).offset(pagesize * (page-1)).limit(pagesize).load::<Movie>(self.conn)?)

    }

    pub fn add(&self, req: NewMovie) -> Result<Movie> {
        use diesel::prelude::*;
        use super::schema::movies::table;

        let sq = diesel::insert_into(table).values(req).execute(self.conn)?;
        return self.show(sq as i32);
    }

    pub fn update(&self, sql_index: i32, req: UpdateMovie) -> Result<Movie> {
        use diesel::prelude::*;
        use super::schema::movies::dsl::*;
        diesel::update(movies.find(sql_index)).set(req).execute(self.conn)?;
        return self.show(sql_index);
    }

    pub fn delete(&self, sql_index:i32) -> Result<()> {
        use diesel::prelude::*;
        use super::schema::movies::dsl::*;
        diesel::delete(movies.find(sql_index)).execute(self.conn)?;
        Ok(())
    }
}

#[derive(Debug, AsChangeset)]
#[table_name="movies"]
pub struct UpdateMovie {
    type_: String,
    titles: String,
    categories: String,
    age_restriction: i32,
}

#[derive(Insertable, Debug)]
#[table_name="movies"]
pub struct NewMovie {
    pub uuid: String,
    pub type_: String,
    pub titles: String,
    pub categories: String,
    pub age_restriction: i32,
    pub img_src: String,

}