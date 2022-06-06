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
) -> Json<Vec<Movie>> {
    return Json(
        services
            .movie_service
            .show_page(
                &search_identifier.get_query(),
                search_identifier.get_page(),
                search_identifier.get_limit(),
            )
            .unwrap(),
    );
}

#[get("/search_movie/{page}/{limit}")]
async fn search_movie_empty(
    search_identifier: Path<EmptySerach>,
    services: Data<Services>,
) -> Json<Vec<Movie>> {
    return Json(
        services
            .movie_service
            .show_page(
                "",
                search_identifier.get_page(),
                search_identifier.get_limit(),
            )
            .unwrap(),
    );
}

#[post("create_movie")]
async fn create_movie(mut payload: Multipart, services: Data<Services>) -> Json<String> {
    //TODO: figure out to drop payload early to prevent error messages
    let uuid = Uuid::new_v4().to_string();
    let mut type_ = None;
    let mut categories = None;
    let mut titles = None;
    let mut age_restriction = 0;
    let mut fileupload = false;

    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value,
            Err(_) => return Json("Couldn't parse form".to_string()),
        };
        match get_content_type(&field).as_str() {
            "image/jpeg" => {
                if get_name(&field) == "cover" {
                    let filepath = format!("static/covers/{}.jpeg", &uuid);
                    match create_file(field, filepath).await {
                        Ok(_) => fileupload = true,
                        Err(err) => return Json(err),
                    }
                } else {
                    return Json("Invalid input".to_string());
                }
            }
            "application/octet-stream" => match get_name(&field).as_str() {
                "titles" => {
                    titles = match extract_text(field).await {
                        Ok(value) => Some(value),
                        Err(err) => return Json(err.to_string()),
                    }
                }
                "type" => {
                    type_ = match extract_text(field).await {
                        Ok(value) => Some(value),
                        Err(err) => return Json(err.to_string()),
                    }
                }
                "categories" => {
                    categories = match extract_text(field).await {
                        Ok(value) => Some(value),
                        Err(err) => return Json(err.to_string()),
                    }
                }
                "age_restriction" => {
                    age_restriction = match extract_text(field).await {
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
                _ => return Json(format!("Invalid input: {}", get_name(&field))),
            },
            _ => {
                println!("type: {}", get_content_type(&field));
                return Json("Invalid input".to_string());
            }
        }
    }

    if fileupload
        && matches!(&titles, Some(_value))
        && matches!(&type_, Some(_value))
        && matches!(&categories, Some(_value))
    {
        match services.movie_service.add(NewMovie {
            uuid,
            type_: type_.unwrap(),
            titles: titles.unwrap(),
            categories: categories.unwrap(),
            age_restriction,
        }) {
            Ok(_) => return Json("200".to_string()),
            Err(err) => return Json(err.to_string()),
        }
    } else {
        if fileupload {
            match remove_file(format!("static/covers/{}.jpeg", &uuid)) {
                Ok(_) => (),
                Err(e) => return Json(format!("failed to cleanup cover for uuid {}  {}", uuid, e)),
            }
        }
        return Json("Payload inclompleted".to_string());
    }
}
