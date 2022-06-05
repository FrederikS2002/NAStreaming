use actix_web::{web::Json, post, get};
use serde::Deserialize;
use std::{collections::HashMap, time::Instant};

#[derive(Deserialize)]
struct updateProgress {
    user: String, 
    episode: String, 
    time: i64,
}

struct ProgressHashmap {
    map: HashMap<String, MovieProgress>
}
impl ProgressHashmap {
    pub fn update(&mut self, user:String, episode: String, time: u64) -> () {
        let timestamp = Instant::now();

        match self.map.get_mut(&user){
            Some(item) => {
                if item.episode != episode {
                    //TODO: push to db
                }else {
                    item.time = time;
                    item.last_update = timestamp;
                    item.watchtime = item.last_update.elapsed().as_secs() + item.watchtime;
                }
            }
            None => {
                self.map.insert(user, MovieProgress{episode, time, last_update: timestamp, watchtime: 0});
            },
        }
    }

    pub fn commit(&mut self, user:String) -> Result<(), String> { 
        match self.map.get(&user) {
            Some(value) => {
                self.map.remove(&user);
                //TODO: add to db
                Ok(())
            },
            None => Err("no data".to_string()) 
        }
    }

    pub fn check_age(&mut self, max_age:u64) -> () {
        let mut removing:Vec<String> = Vec::new();
        for(user, info) in &self.map {
            if info.last_update.elapsed().as_secs() >= max_age {
                removing.push(user.to_string());
                //TODO: update to db;
            }
        }
        for i in removing {
            self.map.remove(i.as_str());
        }
    } 
}

struct MovieProgress {
    episode: String, 
    time: u64,
    last_update: Instant,
    watchtime: u64,
}

#[post("/update_progress")]
async fn update_progress(data: Json<updateProgress>) -> Json<String> {
    return Json("".to_string());
}
#[get("/save_progress")]
async fn commit_progress() -> Json<String> {
    return Json("".to_string());
}
