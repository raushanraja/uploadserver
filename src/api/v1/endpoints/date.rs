use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};

/// Represents a date range with start and end times in UTC.
#[derive(Debug, Serialize, Deserialize)]
struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: Vec<TempFile>,
}

type DateRangeJSON = web::Json<DateRange>;

/// Handles POST requests to get UTC date range.
/// Logs the received date range and returns a success message.
/// Note: From the frontend, send `new Date().toISOString()` for date fields.
#[actix_web::post("/")]
pub async fn get_utc(date_range: DateRangeJSON) -> impl Responder {
    info!("Received date range: {:?}", date_range);
    HttpResponse::Ok().body("Request processed successfully!")
}

#[actix_web::post("/save")]
async fn save_file(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let mut count = 0;

    for mut f in form.file {
        let path = format!("tmp/{}", f.file_name.unwrap());
        log::info!("Saving file to: {}", path);
        let mut newfile = std::fs::File::create(&path).unwrap();
        std::io::copy(&mut f.file, &mut newfile).unwrap();
        count += 1;
    }
    HttpResponse::Ok().body(format!("{} files saved", count))
}
