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
use crate::AppState;

pub async fn serve_file(
    Path(filename): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    method: Method, // adicionado
) -> impl IntoResponse {
    let mut file_path = (*state.media_dir).clone();
    file_path.push(&filename);

    if !file_path.exists() {
        let mut found: Option<PathBuf> = None;
        if let Ok(mut entries) = tokio::fs::read_dir(&*state.media_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem == filename {
                        found = Some(path);
                        break;
                    }
                }
            }
        }

        if let Some(path) = found {
            file_path = path;
        } else {
            return (StatusCode::NOT_FOUND, "File Not Found").into_response();
        }
    }

    let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
    let metadata = tokio::fs::metadata(&file_path).await.unwrap();
    let file_size = metadata.len();

    // Se for HEAD, retorna só os headers
    if method == Method::HEAD {
        return Response::builder()
            .header(header::CONTENT_TYPE, mime.to_string())
            .header(header::CONTENT_LENGTH, file_size.to_string())
            .header("Access-Control-Allow-Origin", "*") 
            .header(header::ACCEPT_RANGES, "bytes")
            .status(StatusCode::OK)
            .body(boxed(Full::from(Bytes::new())))
            .unwrap();
    }

    // Se for vídeo e tiver Range
    let is_video = mime.type_() == mime::VIDEO;
    if is_video {
        if let Some(range) = headers.get(header::RANGE) {
            if let Ok(range_str) = range.to_str() {
                return stream_with_range(file_path, range_str, mime).await;
            }
        }
    }

    // resposta padrão para GET
    let file = File::open(&file_path).await.unwrap();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Response::builder()
        .header(header::CONTENT_TYPE, mime.to_string())
        .header(header::CONTENT_LENGTH, file_size.to_string())
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
    let mut created_files = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        // clone o nome do arquivo imediatamente
        let original_ext = field
            .file_name()
            .and_then(|s| std::path::Path::new(s).extension())
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let data = field.bytes().await.unwrap(); // consome o Field

        // Descobre o próximo número
        let mut max_num = 0;
        if let Ok(mut entries) = tokio::fs::read_dir(&*state.media_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(name) = entry.file_name().to_str() {
                    let stem = std::path::Path::new(name)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    if let Ok(num) = stem.parse::<u64>() {
                        if num > max_num {
                            max_num = num;
                        }
                    }
                }
            }
        }

        let next_num = max_num + 1;

        let mut file_path = (*state.media_dir).clone();
        if original_ext.is_empty() {
            file_path.push(next_num.to_string());
        } else {
            file_path.push(format!("{}.{}", next_num, original_ext));
        }

        // Cria o novo arquivo
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

        created_files.push(file_path.file_name().unwrap().to_string_lossy().to_string());
    }

    if created_files.is_empty() {
        (StatusCode::BAD_REQUEST, "No file uploaded").into_response()
    } else {
        (
            StatusCode::CREATED,
            format!("Files uploaded successfully: {:?}", created_files),
        )
            .into_response()
    }
}