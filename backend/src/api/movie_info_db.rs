use crate::services::Services;
use actix_web::{get, web::Data, web::Json, web::Path, HttpResponse, Responder, Result};
use serde::Serialize;

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

#[derive(Serialize, Debug)]
struct MovieInfoEpi {
    uuid: String,
    epi: i32,
    title: String,
    description: String,
    thumb: String,
    progress: u8,
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
    let icon = "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/8B541716B80EDE784F939632DD0C1FFF6BA18B918361D58383B10228E46EA523/scale?width=1920&aspectRatio=1.78&format=png".to_string();
    let thumb = "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/FA067D32C9A564BE1736D4087008BBB83658102AA4F9FF785A304C2454BAF526/scale?width=2880&aspectRatio=1.78&format=jpeg".to_string();
    let color = [0, 0, 35];
    let description = "abcd".to_string();
    let mut epilist = vec![];
    epilist.push(MovieInfoEpi {uuid:"abcd".to_string(), epi: 1, title: "tit".to_string(), description: "le".to_string(), thumb: "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/D62F35AB392E56542D9E7F541B55F2E79D75B95ADE5F1578BD91FC5A7959CA83/scale?width=400&aspectRatio=NaN&format=jpeg".to_string(), progress: 27});
    epilist.push(MovieInfoEpi {uuid: "efgh".to_string(), epi: 2, title: "tit".to_string(), description: "le".to_string(), thumb: "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/D62F35AB392E56542D9E7F541B55F2E79D75B95ADE5F1578BD91FC5A7959CA83/scale?width=400&aspectRatio=NaN&format=jpeg".to_string(), progress: 27});
    epilist.push(MovieInfoEpi {uuid: "efgh".to_string(), epi: 3, title: "tit".to_string(), description: "le".to_string(), thumb: "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/D62F35AB392E56542D9E7F541B55F2E79D75B95ADE5F1578BD91FC5A7959CA83/scale?width=400&aspectRatio=NaN&format=jpeg".to_string(), progress: 27});
    epilist.push(MovieInfoEpi {uuid: "efgh".to_string(), epi: 4, title: "tit".to_string(), description: "le".to_string(), thumb: "https://prod-ripcut-delivery.disney-plus.net/v1/variant/disney/D62F35AB392E56542D9E7F541B55F2E79D75B95ADE5F1578BD91FC5A7959CA83/scale?width=400&aspectRatio=NaN&format=jpeg".to_string(), progress: 27});
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
