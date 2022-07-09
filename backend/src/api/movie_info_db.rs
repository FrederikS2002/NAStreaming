use crate::{errors::ApiError, models::movie_detail::MovieDetails, services::Services};
use actix_web::{get, web::Data, web::Json, web::Path, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct MovieInfoData {
    uuid: String,
    titles: Vec<String>,
    icon: String,
    thumb: String,
    description: String,
    epilist: Vec<MovieInfoEpi>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MovieInfoEpi {
    uuid: String,
    epi: i32,
    title: String,
    description: String,
    thumb: String,
}

#[get("/movie_detail/{uuid}")]
async fn movie_detail(
    search_identifier: Path<String>,
    services: Data<Services>,
) -> Result<impl Responder, ApiError> {
    let movie = services
        .get_movie_service()
        .show_uuid(search_identifier.to_string());
    let titles = match &movie.unwrap().get(0) {
        Some(v) => v
            .titles
            .clone()
            .split(",")
            .map(|c| c.to_string())
            .collect::<Vec<String>>(),
        None => vec!["".to_string()],
    };
    let detailstemp = services
        .get_movie_details_service()
        .get(search_identifier.to_string())
        .map_err(ApiError::db_error)?;
    let details = match detailstemp.get(0) {
        Some(v) => v,
        None => return Err(ApiError::db_error("no movie in db")),
    };
    let epilist_temp = services
        .get_movie_filelocation_service()
        .show_movie_loc_multi(search_identifier.to_string())
        .map_err(ApiError::db_error)?;
    let epilist: Vec<MovieInfoEpi> =
        serde_json::from_str(&serde_json::to_string(&epilist_temp).unwrap()).unwrap();
    drop(epilist_temp);
    let icon = &details.icon.clone();
    let thumb = &details.thumb.clone();
    let description = &details.description.clone();
    let json = Json(MovieInfoData {
        uuid: search_identifier.to_string(),
        titles,
        icon: icon.to_string(),
        thumb: thumb.to_string(),
        description: description.to_string(),
        epilist,
    });
    Ok(json)
}
