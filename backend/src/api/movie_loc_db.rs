use crate::{
    errors::{ApiError, ApiErrrorType},
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
async fn epi_data(
    payload: Path<EpiDataReciver>,
    services: Data<Services>,
) -> Result<Json<EpiData>, ApiError> {
    let loc = services
        .get_movie_filelocation_service()
        .show_movie_loc_single(payload.movie.clone(), payload.episode.clone())
        .map_err(ApiError::db_error)?;
    let loc = match loc.get(0) {
        Some(v) => v,
        None => {
            return Err(ApiError::db_error("no movie in db"));
        }
    };
    let next = services
        .get_movie_filelocation_service()
        .show_next_movie(loc.movie.clone(), loc.epi)
        .map_err(ApiError::db_error)?;
    let next = match next.get(0) {
        Some(v) => v.uuid.clone(),
        None => "".to_string(),
    };
    Ok(Json(EpiData {
        movie: payload.movie.clone(),
        file: loc.filename.clone(),
        next,
        progress: 0,
    }))
}

#[post("upload_episodes")]
async fn upload_episodes(
    mut payload: Multipart,
    services: Data<Services>,
) -> Result<Json<String>, ApiError> {
    let uuid = Uuid::new_v4().to_string();
    let mut title = "".to_string();
    let mut fileupload = false;
    let mut movie = None;
    let mut epi = 0;
    let mut description = "".to_string();
    let mut filename = None;
    let mut filepath = None;

    while let Some(item) = payload.next().await {
        let field = item.map_err(|err| ApiError {
            message: Some("couldnt parse field".to_string()),
            cause: Some(err.to_string()),
            err_type: crate::errors::ApiErrrorType::ReadError,
        })?;
        match get_content_type(&field).as_str() {
            "video/mp4" => {
                if get_name(&field) == "file" {
                    filename = Some(format!("{}-{}", &uuid, get_filename(&field).unwrap()));
                    //TODO: Make sure that movie is known
                    filepath = Some(format!("static/movies/{}", movie.as_ref().unwrap()));
                    std::fs::create_dir_all(&filepath.as_ref().unwrap())
                        .map_err(ApiError::write_error)?;
                    create_file(
                        field,
                        format!(
                            "{}/{}",
                            &filepath.as_ref().unwrap(),
                            &filename.as_ref().unwrap()
                        ),
                    )
                    .await
                    .map_err(ApiError::write_error)?;
                    fileupload = true;
                } else {
                    return Err(ApiError::invalid_input_error(format!(
                        "expeced file found => {}",
                        get_name(&field)
                    )));
                }
            }
            "application/octet-stream" => match get_name(&field).as_str() {
                "movie" => {
                    movie = Some(extract_text(field).await.map_err(ApiError::read_error)?);
                }
                "epi" => {
                    epi = match extract_text(field).await {
                        Ok(value) => {
                            if value.parse::<i32>().is_ok() {
                                value.parse::<i32>().unwrap()
                            } else {
                                return Err(ApiError::invalid_input_error(
                                    "expected integer found => string".to_string(),
                                ));
                            }
                        }
                        Err(err) => return Err(ApiError::read_error(err)),
                    }
                }
                "name" => {
                    title = extract_text(field).await.map_err(ApiError::read_error)?;
                }
                "description" => {
                    description = extract_text(field).await.map_err(ApiError::read_error)?;
                }
                _ => {
                    return Err(ApiError::invalid_input_error(format!(
                        "unexpected input type => {}",
                        get_content_type(&field)
                    )))
                }
            },
            _ => {
                return Err(ApiError::invalid_input_error(format!(
                    "unexpected input type => {}",
                    get_content_type(&field)
                )))
            }
        }
    }

    if fileupload && epi != 0 && matches!(&movie, Some(_value)) && matches!(&filename, Some(_value))
    {
        //TODO: Delete file on error db error
        services
            .get_movie_filelocation_service()
            .add(NewMovieFileLoation {
                uuid,
                movie: movie.unwrap(),
                epi,
                title,
                filename: filename.unwrap(),
                description,
                thumb: "".to_string(),
            })
            .map_err(ApiError::db_error)?;
        Ok(Json("upload complete".to_string()))
    } else {
        if fileupload {
            remove_file(format!("{}/{}", &filepath.unwrap(), &filename.unwrap())).map_err(|err| ApiError {
                message: Some(format!("failed to cleanup cover for uuid {}", uuid)),
                cause: Some(err.to_string()),
                err_type: ApiErrrorType::WriteError,
            })?;

        }
        return Err(ApiError::invalid_input_error("Payload inclompleted".to_string()));
        
    }
}
