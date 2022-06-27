use crate::{
    handle_field::{create_file, extract_text, get_content_type, get_filename, get_name},
    models::movie_location::NewMovieFileLoation,
    services::Services,
};
use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::Json,
    web::{Data, Path},
};
use futures_util::stream::StreamExt as _;
use serde::{Deserialize, Serialize};
use std::fs::remove_file;
use uuid::Uuid;

#[derive(Serialize)]
struct EpiData {
    movie: String,
    next: String,
    file: String,
    progress: u64,
}

#[derive(Serialize, Deserialize)]
struct EpiDataReciver {
    movie: String,
    episode: String,
}
#[get("epi_data/{movie}/{episode}")]
async fn epi_data(payload: Path<EpiDataReciver>, services: Data<Services>) -> Json<EpiData> {
    println!("test");
    let loc = services
        .get_movie_filelocation_service()
        .show_movie_loc_single(payload.movie.clone(), payload.episode.clone())
        .unwrap();
    let loc = match loc.get(0) {
        Some(v) => v,
        None => {
            return Json(EpiData {
                movie: "".to_string(),
                file: "".to_string(),
                next: "".to_string(),
                progress: 0,
            })
        }
    };
    let next = services
        .get_movie_filelocation_service()
        .show_next_movie(loc.movie.clone(), loc.epi)
        .unwrap();
    let next = match next.get(0) {
        Some(v) => v.uuid.clone(),
        None => "".to_string(),
    };
    return Json(EpiData {
        movie: payload.movie.clone(),
        file: loc.filename.clone(),
        next,
        progress: 0,
    });
}

#[post("upload_episodes")]
async fn upload_episodes(mut payload: Multipart, services: Data<Services>) -> Json<String> {
    let uuid = Uuid::new_v4().to_string();
    let mut title = "".to_string();
    let mut fileupload = false;
    let mut movie = None;
    let mut epi = 0;
    let mut description = "".to_string();
    let mut filename = None;
    let mut filepath = None;

    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value,
            Err(_) => return Json("Couldn't parse form".to_string()),
        };
        match get_content_type(&field).as_str() {
            "video/mp4" => {
                if get_name(&field) == "file" {
                    filename = Some(format!("{}-{}", &uuid, get_filename(&field).unwrap()));
                    //TODO: Make sure that movie is known
                    filepath = Some(format!("static/movies/{}", movie.as_ref().unwrap()));
                    match std::fs::create_dir_all(&filepath.as_ref().unwrap()) {
                        Ok(_) => (),
                        Err(err) => println!("{:?}", err),
                    }
                    match create_file(
                        field,
                        format!(
                            "{}/{}",
                            &filepath.as_ref().unwrap(),
                            &filename.as_ref().unwrap()
                        ),
                    )
                    .await
                    {
                        Ok(_) => (),
                        Err(err) => return Json(err),
                    }

                    fileupload = true;
                } else {
                    return Json("Invalid input".to_string());
                }
            }
            "application/octet-stream" => match get_name(&field).as_str() {
                "movie" => {
                    movie = match extract_text(field).await {
                        Ok(value) => Some(value),
                        Err(err) => return Json(err.to_string()),
                    }
                }
                "epi" => {
                    epi = match extract_text(field).await {
                        Ok(value) => {
                            if value.parse::<i32>().is_ok() {
                                value.parse::<i32>().unwrap()
                            } else {
                                return Json("Invalid inpu".to_string());
                            }
                        }
                        Err(err) => return Json(err.to_string()),
                    }
                }
                "name" => {
                    title = match extract_text(field).await {
                        Ok(value) => value,
                        Err(err) => return Json(err.to_string()),
                    }
                }
                "description" => {
                    description = match extract_text(field).await {
                        Ok(value) => value,
                        Err(err) => return Json(err.to_string()),
                    }
                }
                _ => {
                    return Json("Invalid input".to_string());
                }
            },
            _ => {
                println!("type: {}", get_content_type(&field));
                return Json("Invalid input".to_string());
            }
        }
    }

    if fileupload && epi != 0 && matches!(&movie, Some(_value)) && matches!(&filename, Some(_value))
    {
        //TODO: Delete file on error db error
        match services
            .get_movie_filelocation_service()
            .add(NewMovieFileLoation {
                uuid,
                movie: movie.unwrap(),
                epi,
                title,
                filename: filename.unwrap(),
                description,
                thumb: "".to_string(),
            }) {
            Ok(_) => return Json("200".to_string()),
            Err(err) => return Json(err.to_string()),
        }
    } else {
        if fileupload {
            match remove_file(format!("{}/{}", &filepath.unwrap(), &filename.unwrap())) {
                Ok(_) => (),
                Err(e) => return Json(format!("failed to cleanup cover for uuid {}  {}", uuid, e)),
            }
        }
        return Json("Payload inclompleted".to_string());
    }
}
