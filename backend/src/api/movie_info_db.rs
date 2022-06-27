use crate::services::Services;
use actix_web::{get, web::Data, web::Json, web::Path, HttpResponse, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
struct MovieInfoData {
    uuid: String,
    titles: Vec<String>,
    icon: String,
    thumb: String,
    color: [u8; 3],
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
) -> Result<impl Responder, actix_web::Error> {
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
    let icon = "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/F180E12F2C8DAC50ACCD15197541CAEE53509C068E977C8F8EA8B5F962073994/scale?width=1920&aspectRatio=1.78&format=png".to_string();
    let thumb = "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/DB7CE36F697D9269E5B6E649CE6E963E1A1C4BDF37A529AAD2B5B3395164FABF/scale?width=2880&aspectRatio=1.78&format=jpeg".to_string();
    let color = [0, 0, 35];
    let description = "abcd".to_string();
    let epilist_temp = services.get_movie_filelocation_service().show_movie_loc_multi(search_identifier.to_string()).unwrap();
    let epilist:Vec<MovieInfoEpi> =  serde_json::from_str(&serde_json::to_string(&epilist_temp).unwrap()).unwrap();
    drop(epilist_temp);
    let json = Json(MovieInfoData {
        uuid: search_identifier.to_string(),
        titles,
        icon,
        thumb,
        color,
        description,
        epilist,
    });
    Ok(json)
    // return HttpResponse::Ok().body(
}
