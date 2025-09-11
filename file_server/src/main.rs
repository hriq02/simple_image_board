use std::{path::PathBuf, sync::Arc};
use axum::{
    http::HeaderValue, middleware::Next, response::Response, routing::{get, post}, Router, Server
};
use axum::http::Method;

mod file_router;

#[derive(Clone)]
struct AppState {
    media_dir: Arc<PathBuf>,
}

// Middleware CORS manual
async fn cors<B>(req: axum::http::Request<B>, next: Next<B>) -> Response {

    if req.method() == Method::OPTIONS {
        let mut res = Response::new(axum::body::BoxBody::default());
        let headers = res.headers_mut();
        headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET,POST,OPTIONS"));
        headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));
        return res;
    }

    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET,POST,OPTIONS"));
    headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));

    response
}

#[tokio::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H"); // clear terminal
    println!("File Server started on http://localhost:7000/");

    let app = Router::new()
        .route("/:filename", get(file_router::serve_file))
        .route("/", post(file_router::insert_new_file))
        .with_state(AppState {
            media_dir: Arc::new(PathBuf::from("/home/hriq/repos/simple_image_board/resources/")),
        })
        .layer(axum::middleware::from_fn(cors)); // <-- middleware CORS manual

    Server::bind(&"0.0.0.0:7000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}