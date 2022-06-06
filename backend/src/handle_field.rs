use actix_multipart::Field;
use actix_web::web::block;
use anyhow::Result;
use futures_util::stream::StreamExt as _;
use std::io::Write;

pub async fn create_file(mut field: Field, filepath: String) -> Result<(), String> {
    if std::path::Path::new(&filepath).exists() {
        return Err("file exists".to_string());
    }
    let mut file = match std::fs::File::create(filepath) {
        Ok(file) => file,
        Err(_) => return Err("Failed to create file".to_string()),
    };
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        match block(move || file.write_all(&chunk.unwrap()).map(|_| file)).await {
            Ok(result) => match result {
                Ok(new_file) => file = new_file,
                Err(_) => return Err("failed".to_string()),
            },
            Err(_) => return Err("failed".to_string()),
        };
    }
    return Ok(());
}

pub fn get_name(field: &Field) -> String {
    return field.name().to_string();
}

pub fn get_filename(field: &Field) -> Result<String, ()> {
    match field.content_disposition().get_filename() {
        Some(filename) => return Ok(filename.replace(' ', "-").to_string()),
        None => return Err(()),
    };
}

pub fn get_content_type(field: &Field) -> String {
    return field.content_type().to_string();
}

pub async fn extract_text(mut field: Field) -> Result<String> {
    let mut result: String = "".to_string();
    let mut count = 0;
    while let Some(chunk) = field.next().await {
        if count > 1 {
            println!("multiple chunks");
        }
        count += 1;
        result = format!("{}{}", result, std::str::from_utf8(&chunk?)?.to_string());
    }
    Ok(result)
}
