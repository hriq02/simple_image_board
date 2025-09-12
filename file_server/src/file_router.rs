use axum::{
    body::{boxed, BoxBody, Bytes, Full, StreamBody},
    extract::{Path, State},
    http::{header, HeaderMap, Method, Response, StatusCode},
    response::IntoResponse,
};
use mime_guess::mime;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio_util::io::ReaderStream;
use axum::extract::multipart::Multipart;
use tokio::io::AsyncWriteExt;
use crate::{file_register::{get_file_type, FileMetadata, FileType}, AppState};

pub async fn serve_file(
    Path(filename): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    method: Method,
) -> impl IntoResponse {
    let file_register = state.file_register.lock().await;
    
    let id : u64 = match filename.parse(){
        Ok(vl) => vl,
        Err(_) => {
            return (StatusCode::NOT_FOUND, format!("File Not Found: {}" , filename)).into_response();
        }
    };

    if !file_register.contains(id) {
        return (StatusCode::NOT_FOUND, format!("File Not Found: {}" , id)).into_response();
    }

    
    let file_metadata = match file_register.get(id){
        Some(file) => file,
        None => {
            return (StatusCode::NOT_FOUND, format!("File Not Found: {}" , filename)).into_response();
        }
    };
    
    let file_path = PathBuf::from(file_metadata.file_path.clone()); 


    let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
    if method == Method::HEAD {
        return Response::builder()
            .header(header::CONTENT_TYPE, mime.to_string())
            .header(header::CONTENT_LENGTH, file_metadata.file_size.to_string())
            .header("Access-Control-Allow-Origin", "*") 
            .header(header::ACCEPT_RANGES, "bytes")
            .status(StatusCode::OK)
            .body(boxed(Full::from(Bytes::new())))
            .unwrap();
    }

    
    if file_metadata.file_type == FileType::Video {
        if let Some(range) = headers.get(header::RANGE) {
            if let Ok(range_str) = range.to_str() {
                return stream_with_range(file_path, range_str, mime).await;
            }
        }
    }

    let file = File::open(&file_path).await.unwrap();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Response::builder()
        .header(header::CONTENT_TYPE, mime.to_string())
        .header(header::CONTENT_LENGTH, file_metadata.file_size.to_string())
        .header("Access-Control-Allow-Origin", "*") 
        .header(header::ACCEPT_RANGES, "bytes")
        .body(boxed(body))
        .unwrap()
}

pub async fn stream_with_range(
    path: PathBuf,
    range_header: &str,
    mime: mime::Mime,
) -> Response<BoxBody>{
    use tokio::io::{AsyncReadExt, AsyncSeekExt};

    let mut file = File::open(&path).await.unwrap();
    let metadata = tokio::fs::metadata(&path).await.unwrap();
    let file_size = metadata.len();

    let start: u64 = range_header
        .replace("bytes=", "")
        .replace("-", "")
        .parse()
        .unwrap_or(0);

    let mut buffer = Vec::new();
    file.seek(std::io::SeekFrom::Start(start)).await.unwrap();
    file.read_to_end(&mut buffer).await.unwrap();

    Response::builder()
        .status(StatusCode::PARTIAL_CONTENT)
        .header(header::CONTENT_TYPE, mime.to_string())
        .header("Access-Control-Allow-Origin", "*") 
        .header(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", start, file_size - 1, file_size),
        )
        .header(header::ACCEPT_RANGES, "bytes")
        .body(boxed(Full::from(Bytes::from(buffer))))
        .unwrap()
}




pub async fn insert_new_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {

    let mut file_register = state.file_register.lock().await;

    let field = match multipart.next_field().await {
        Ok(Some(field)) => field,
        Ok(None) => return (
            StatusCode::BAD_REQUEST,
            "No file uploaded",
        ).into_response(),
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read file: {}", e),
        ).into_response(),
    };

    let original_ext = field
        .file_name()
        .and_then(|s| std::path::Path::new(s).extension())
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    if original_ext.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "No file uploaded",
        ).into_response();
    }

    let data = match field.bytes().await{
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read file: {}", e),
            ).into_response();
        }
    };

    let next_num = match get_next_id().await{
        Ok(num) => num,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get next post id: {}", e),
            ).into_response();
        }
    };
    println!("new file created with the id: {}", next_num);
    
    file_register.insert(next_num, FileMetadata{
        file_path: next_num.to_string(),
        extension: original_ext.clone(),
        file_size: data.len() as u64,
        file_type: get_file_type(&original_ext)
    });

    let file_path = PathBuf::from(format!("{}/{}.{}", file_register.folder.to_string_lossy(), next_num, original_ext));

    match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)
        .await
    {
        Ok(mut f) => {
            if let Err(err) = f.write_all(&data).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to write file: {}", err),
                )
                    .into_response();
            }
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create file: {}", err),
            )
                .into_response();
        }
    }

    (
        StatusCode::CREATED,
        "File uploaded successfully",
    ).into_response()

}




async fn get_next_id() -> Result<u64,Box<dyn std::error::Error> >{

    match 
        reqwest::get("http://localhost:8000/next_post_id").await?
        .text().await?
        .parse()
    {
        Ok(num) => Ok(num),
        Err(e) => Err(Box::new(e))
    }
}