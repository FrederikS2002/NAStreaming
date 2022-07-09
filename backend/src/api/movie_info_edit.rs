use crate::{
    handle_field::{create_file, get_name},
    services::Services,
};
use actix_multipart::Multipart;
use actix_web::{post, web::Data, web::Json};
use futures_util::stream::StreamExt as _;

#[post("upload_files")]
async fn rewrite_data(mut payload: Multipart, services: Data<Services>) -> Json<String> {
    let mut uuid = "";
    let mut fileupload = false;
    let mut filepath = "";
    let mut ending = "";

    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value,
            Err(_) => return Json("Couldn't parse form".to_string()),
        };
        //TODO: add check for right type
        match get_name(&field).as_ref() {
            "trailer" => {
                filepath = "static/trailer";
                ending = "mp4";
            }
            "thumb" => {
                filepath = "static/thumb";
                ending = "png";
            }
            "icon" => {
                filepath = "static/icon";
                ending = "png";
            }
            "covers" => {
                filepath = "static/covers";
                ending = "jpeg";
            }
            _ => return Json("Invalid input".to_string()),
        }
        match std::fs::create_dir_all(&uuid) {
            Ok(_) => (),
            Err(err) => return Json(err.to_string()),
        }
        match create_file(field, format!("{}/{}.{}", filepath, uuid, ending)).await {
            Ok(_) => (),
            Err(err) => return Json(err),
        }
    }
    //TODO: update db && return
    return Json("".to_string());
}
