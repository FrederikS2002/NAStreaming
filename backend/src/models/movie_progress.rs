use crate::schema::*;
use anyhow::Result;
use diesel::{prelude::*, MysqlConnection};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct MovieProgress {
    id: i32,
    uuid: String,
    progress: i32,
    user: String,
}

pub struct MovieProgressService<'a> {
    pub conn: &'a MysqlConnection,
}

impl<'a> MovieProgressService<'a> {
    pub fn update(&self, data: NewMovieProgress) -> bool {
        use crate::schema::movie_progress::{
            dsl::{movie_progress, uuid},
            table,
        };
        match movie_progress
            .filter(uuid.eq(&data.uuid))
            .limit(1)
            .load::<MovieProgress>(self.conn)
        {
            Ok(val) => {
                match diesel::update(movie_progress.find(val[0].id))
                    .set(UpdateMovieProgress {
                        progress: data.progress,
                    })
                    .execute(self.conn)
                {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            Err(_) => match diesel::insert_into(table).values(data).execute(self.conn) {
                Ok(_) => true,
                Err(_) => false,
            },
        }
    }

    pub fn get(&self, uuid: String, user: String) -> Result<MovieProgress> {
        use crate::schema::movie_progress::dsl::{movie_progress, uuid};
        Ok(movie_progress
            .filter(uuid.eq(user))
            .limit(1)
            .load::<MovieProgress>(self.conn)?)
    }
}

#[derive(Insertable, Debug)]
#[table_name = "movie_progress"]
pub struct NewMovieProgress {
    uuid: String,
    progress: i32,
    user: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "movie_progress"]
pub struct UpdateMovieProgress {
    progress: i32,
}
