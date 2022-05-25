use actix_web::{get, web::Path, web::Json, web::Data};
use super::super::{models, mysql};
use serde::{Serialize, Deserialize};
use diesel::MysqlConnection;

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
