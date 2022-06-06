use crate::schema::*;
use anyhow::Result;
use diesel::{prelude::*, MysqlConnection};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct MovieWatchtime {
    id: i32,
    user: String,
    watchtime: i32,
}

pub struct MovieWatchtimeService<'a> {
    pub conn: &'a MysqlConnection,
}

impl<'a> MovieWatchtimeService<'a> {
    pub fn update(&self, userl: String, watchtime: i32) -> bool {
        use crate::schema::movie_watchtime::{dsl::movie_watchtime, dsl::user, table};
        match movie_watchtime
            .filter(user.eq(&userl))
            .limit(1)
            .load::<MovieWatchtime>(self.conn)
        {
            Ok(value) => {
                return match diesel::update(movie_watchtime.find(value[0].id))
                    .set(UpdateWatchtime {
                        watchtime: value[0].watchtime + watchtime,
                    })
                    .execute(self.conn)
                {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            Err(_) => {
                return match diesel::insert_into(table)
                    .values(NewMovieWatchtime {
                        user: userl,
                        watchtime,
                    })
                    .execute(self.conn)
                {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        }
    }

    pub fn get(&self, userl: String) -> Result<Vec<MovieWatchtime>> {
        use crate::schema::movie_watchtime::dsl::{movie_watchtime, user};
        return Ok(movie_watchtime
            .filter(user.eq(userl))
            .limit(1)
            .load::<MovieWatchtime>(self.conn)?);
    }
}

#[derive(Debug, AsChangeset)]
#[table_name = "movie_watchtime"]
pub struct UpdateWatchtime {
    watchtime: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "movie_watchtime"]
pub struct NewMovieWatchtime {
    pub user: String,
    pub watchtime: i32,
}
