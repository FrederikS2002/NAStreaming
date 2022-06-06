use actix_web::{get, post, web::Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateProgress {
    user: String,
    episode: String,
    time: i64,
}

#[post("/update_progress")]
async fn updateProgress(data: Json<UpdateProgress>) -> Json<String> {
    return Json(format!("{:?}", data));
}
#[get("/save_progress")]
async fn commit_progress() -> Json<String> {
    return Json("".to_string());
}
