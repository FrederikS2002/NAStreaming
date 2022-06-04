use actix_web::{post, web::Json, web::Data};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use diesel::MysqlConnection;
use crate::handle_field::{get_content_type, get_name, create_file, get_filename};

#[post("upload_episodes")]
async fn upload_episodes(mut payload: Multipart, conn: Data<MysqlConnection>) -> Json<String> {
    let mut fileupload = false;
    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value, 
            Err(_) => return Json("Couldn't parse form".to_string())
        }; 
        match get_content_type(&field).as_str() {
            "video/mp4" => {
                if get_name(&field) == "file" {
                    let filepath = format!("static/covers/{}.mp4", get_filename(&field).unwrap());
                    match create_file(field, filepath).await {
                        Ok(_) => fileupload = true,
                        Err(err) => return Json(err)
                    }
                }else {
                    return Json("Invalid input".to_string());
                }
            }
            _ => {
                println!("type: {}", get_content_type(&field));
            }
        }
 
    }
    return Json("200".to_string());
}
