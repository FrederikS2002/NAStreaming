use actix_web::{get, post, web::Path, web::Json, web::Data};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use models::NewMovie;
use crate::models;
use serde::{Serialize, Deserialize};
use diesel::MysqlConnection;
use std::fs::remove_file;
use uuid::Uuid;
use crate::handle_field::{get_content_type, get_name, create_file, extract_text};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchIdentifier {
    pub query: String,
    pub page: String,
    pub limit: String
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
pub async fn search_movie(search_identifier: Path<SearchIdentifier>, conn: Data<MysqlConnection>) -> Json<Vec<models::Movie>> {
    let testing = super::super::models::MovieService{conn: &conn}; 
    return Json(testing.show_page(&search_identifier.get_query(), search_identifier.get_page(), search_identifier.get_limit()).unwrap());
}

#[get("/search_movie/{page}/{limit}")]
pub async fn search_movie_empty(search_identifier: Path<EmptySerach>, conn: Data<MysqlConnection>) -> Json<Vec<models::Movie>> {
    let testing = super::super::models::MovieService{conn: &conn};
    return Json(testing.show_page("", search_identifier.get_page(), search_identifier.get_limit()).unwrap());
}

#[post("create_movie")]
async fn create_movie(mut payload: Multipart, conn: Data<MysqlConnection>) -> Json<String> {
    //TODO: figure out to drop payload early to prevent error messages
    let uuid = Uuid::new_v4().to_string(); 
    let mut type_ = None;
    let mut categories = None;
    let mut titles = None;
    let mut age_restriction = None;
    let mut fileupload = false;

    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value, 
            Err(_) => return Json("Couldn't parse form".to_string())
        };
        match get_content_type(&field).as_str() {
            "image/jpeg" => {
                if get_name(&field) == "cover" {
                    let filepath = format!("static/covers/{}.jpeg", &uuid);
                    match create_file(field, filepath).await {
                        Ok(_) => fileupload = true,
                        Err(err) => return Json(err)
                    }
                }else {
                    return Json("Invalid input".to_string());
                }
            }
            "application/octet-stream" => {
                match get_name(&field).as_str() {
                    "titles" => {
                        titles = match extract_text(field).await {
                            Ok(value) => Some(value),
                            Err(err) => return Json(err.to_string())
                        }
                    }
                    "type" => {
                        type_ = match extract_text(field).await {
                            Ok(value) => Some(value),
                            Err(err) => return Json(err.to_string())
                        }
                    }
                    "categories" => {
                        categories = match extract_text(field).await {
                            Ok(value) => Some(value),
                            Err(err) => return Json(err.to_string())
                        }
                    }
                    "age_restriction" => {
                        age_restriction = match extract_text(field).await {
                            Ok(value) => {
                                if value.parse::<i32>().is_ok() {
                                    Some(value.parse::<i32>().unwrap())
                                }else {
                                    return Json("Invalid inpu".to_string())
                                }
                            },
                            Err(err) => return Json(err.to_string())
                        }
                    }
                    _ => return Json(format!("Invalid input: {}", get_name(&field)))
                }
            }
            _ => {
                println!("type: {}", get_content_type(&field));
                return Json("Invalid input".to_string());
            }
            
        }
    }

    if fileupload && matches!(&titles, Some(_value)) && matches!(&type_, Some(_value)) && matches!(&categories, Some(_value)) && matches!(age_restriction, Some(_value)) {
        let testing = super::super::models::MovieService{conn: &conn};
        match testing.add(NewMovie { uuid, type_: type_.unwrap(), titles: titles.unwrap(), categories:categories.unwrap(), age_restriction: age_restriction.unwrap(), }){
            Ok(_) => return Json("200".to_string()),
            Err(err) => return Json(err.to_string())
        }
    } else {
        if fileupload {
            match remove_file(format!("static/covers/{}.jpeg", &uuid)) {
                Ok(_) => (),
                Err(e) => return Json(format!("failed to cleanup cover for uuid {}  {}", uuid, e))
            }
        }
        return Json("Payload inclompleted".to_string());
    } 
}
