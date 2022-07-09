use crate::errors::{ApiError, ApiErrrorType};
use crate::handle_field::{create_file, extract_text, get_content_type, get_name};
use crate::models::movies::{Movie, NewMovie};
use crate::services::Services;
use actix_multipart::Multipart;
use actix_web::{get, post, web::Data, web::Json, web::Path};
use futures_util::stream::StreamExt as _;
use serde::{Deserialize, Serialize};
use std::fs::remove_file;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchIdentifier {
    pub query: String,
    pub page: String,
    pub limit: String,
}

impl SearchIdentifier {
    fn get_query(&self) -> String {
        return self.query.clone();
    }
    fn get_page(&self) -> i64 {
        return self.page.parse::<i64>().unwrap_or(1);
    }
    fn get_limit(&self) -> i64 {
        if self.page.parse::<i64>().is_ok() {
            return self.limit.parse::<i64>().unwrap_or(0);
        }
        return 0;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptySerach {
    pub page: String,
    pub limit: String,
}

impl EmptySerach {
    fn get_page(&self) -> i64 {
        return self.page.parse::<i64>().unwrap_or(1);
    }
    fn get_limit(&self) -> i64 {
        if self.page.parse::<i64>().is_ok() {
            return self.limit.parse::<i64>().unwrap_or(0);
        }
        return 0;
    }
}

#[get("/search_movie/{query}/{page}/{limit}")]
async fn search_movie(
    search_identifier: Path<SearchIdentifier>,
    services: Data<Services>,
) -> Result<Json<Vec<Movie>>, ApiError> {
    let req = services
        .get_movie_service()
        .show_page(
            &search_identifier.get_query(),
            search_identifier.get_page(),
            search_identifier.get_limit(),
        )
        .map_err(ApiError::db_error)?;
    Ok(Json(req))
}

#[get("/search_movie/{page}/{limit}")]
async fn search_movie_empty(
    search_identifier: Path<EmptySerach>,
    services: Data<Services>,
) -> Result<Json<Vec<Movie>>, ApiError> {
    let req = services
        .get_movie_service()
        .show_page(
            "",
            search_identifier.get_page(),
            search_identifier.get_limit(),
        )
        .map_err(ApiError::db_error)?;
    Ok(Json(req))
}

#[post("/create_movie/")]
async fn create_movie(
    mut payload: Multipart,
    services: Data<Services>,
) -> Result<Json<String>, ApiError> {
    //TODO: figure out to drop payload early to prevent error messages
    let uuid = Uuid::new_v4().to_string();
    let mut type_ = None;
    let mut categories = None;
    let mut titles = None;
    let mut age_restriction = 0;
    let mut fileupload = false;

    while let Some(item) = payload.next().await {
        let field = item.map_err(|err| ApiError {
            message: Some("couldnt parse field".to_string()),
            cause: Some(err.to_string()),
            err_type: crate::errors::ApiErrrorType::ReadError,
        })?;

        match get_content_type(&field).as_str() {
            "image/jpeg" => {
                if get_name(&field) == "cover" {
                    let filepath = format!("static/covers/{}.jpeg", &uuid);
                    match create_file(field, filepath).await {
                        Ok(_) => fileupload = true,
                        Err(err) => return Err(ApiError::write_error(err)),
                    }
                } else {
                    return Err(ApiError::invalid_input_error(format!(
                        "expeced cover found => {}",
                        get_name(&field)
                    )));
                }
            }
            "application/octet-stream" => match get_name(&field).as_str() {
                "titles" => {
                    titles = Some(extract_text(field).await.map_err(ApiError::read_error)?);
                }
                "type" => {
                    type_ = Some(extract_text(field).await.map_err(ApiError::read_error)?);
                }
                "categories" => {
                    categories = Some(extract_text(field).await.map_err(ApiError::read_error)?);
                }
                "age_restriction" => {
                    age_restriction = match extract_text(field).await {
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
                _ => {
                    return Err(ApiError::invalid_input_error(format!(
                        "unexpected text input => {}",
                        get_name(&field)
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

    if fileupload
        && matches!(&titles, Some(_value))
        && matches!(&type_, Some(_value))
        && matches!(&categories, Some(_value))
    {
        services
            .get_movie_service()
            .add(NewMovie {
                uuid,
                type_: type_.unwrap(),
                titles: titles.unwrap(),
                categories: categories.unwrap(),
                age_restriction,
            })
            .map_err(ApiError::db_error)?;
        Ok(Json("upload complete".to_string()))
    } else {
        if fileupload {
            remove_file(format!("static/covers/{}.jpeg", &uuid)).map_err(|err| ApiError {
                message: Some(format!("failed to cleanup cover for uuid {}", uuid)),
                cause: Some(err.to_string()),
                err_type: ApiErrrorType::WriteError,
            })?;
        }
        return Err(ApiError::invalid_input_error("Payload inclompleted".to_string()));
    }
}
