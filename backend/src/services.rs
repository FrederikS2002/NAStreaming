use crate::models::{movie_location::MovieFileLoationService, movies::MovieService};
use actix_web::{
    rt::{spawn, time},
    web::Data,
};
use diesel::MysqlConnection;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub fn check_hashmap(services: Data<Services>) {
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            // services.check_progress_age(5);
        }
    });
}
pub struct Services {
    conn: MysqlConnection,
    progress_hashmap: HashMap<String, MovieProgress>,
}

impl Services {
    pub fn new(conn: MysqlConnection) -> Self {
        let progress_hashmap = HashMap::new();
        Self {
            conn,
            progress_hashmap,
        }
    }

    pub fn get_movie_service(&self) -> MovieService {
        return MovieService { conn: &self.conn}
    }

    pub fn get_movie_filelocation_service(&self) -> MovieFileLoationService {
        return MovieFileLoationService { conn: &self.conn }
    }

    pub fn update(&mut self, user: String, episode: String, time: u64) -> () {
        let timestamp = Instant::now();

        match self.progress_hashmap.get_mut(&user) {
            Some(item) => {
                if item.episode != episode {
                    //TODO: push to db
                } else {
                    item.time = time;
                    item.last_update = timestamp;
                    item.watchtime = item.last_update.elapsed().as_secs() + item.watchtime;
                }
            }
            None => {
                self.progress_hashmap.insert(
                    user,
                    MovieProgress {
                        episode,
                        time,
                        last_update: timestamp,
                        watchtime: 0,
                    },
                );
            }
        }
    }

    pub fn commit_progress(&mut self, user: String) -> Result<(), String> {
        match self.progress_hashmap.get(&user) {
            Some(value) => {
                self.progress_hashmap.remove(&user);
                //TODO: add to db
                Ok(())
            }
            None => Err("no data".to_string()),
        }
    }

    pub fn check_progress_age(&mut self, max_age: u64) -> () {
        let mut removing: Vec<String> = Vec::new();
        for (user, info) in &self.progress_hashmap {
            if info.last_update.elapsed().as_secs() >= max_age {
                removing.push(user.to_string());
                //TODO: update to db;
            }
        }
        for i in removing {
            self.progress_hashmap.remove(i.as_str());
        }
    }
}

struct MovieProgress {
    episode: String,
    time: u64,
    last_update: Instant,
    watchtime: u64,
}
