use actix_web::{post, web, HttpResponse, HttpRequest, Error};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use std::io::{Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::path::Path;

#[post("/upload")]
pub async fn upload_image(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let upload_dir = "uploads";
    std::fs::create_dir_all(upload_dir)?;

    let mut file_url = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let file_ext = Path::new(filename).extension().unwrap().to_str().unwrap();
        
        let file_name = format!("{}.{}", Uuid::new_v4(), file_ext);
        let filepath = format!("{}/{}", upload_dir, file_name);

        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&filepath)?;

        // 获取文件大小
        let file_size = f.metadata()?.len();

        // 检查是否有 Range 头
        if let Some(range) = req.headers().get("Range") {
            let range_str = range.to_str().unwrap();
            if let Some(start_byte) = parse_range(range_str) {
                if start_byte < file_size {
                    f.seek(SeekFrom::Start(start_byte))?;
                }
            }
        }

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }

        file_url = format!("/uploads/{}", file_name);
    }

    if !file_url.is_empty() {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "File uploaded successfully",
            "url": file_url
        })))
    } else {
        Ok(HttpResponse::InternalServerError().body("Failed to upload file"))
    }
}

fn parse_range(range: &str) -> Option<u64> {
    let parts: Vec<&str> = range.split('=').collect();
    if parts.len() == 2 && parts[0] == "bytes" {
        let range_parts: Vec<&str> = parts[1].split('-').collect();
        if range_parts.len() >= 1 {
            return range_parts[0].parse::<u64>().ok();
        }
    }
    None
}
