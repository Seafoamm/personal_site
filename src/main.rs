use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::fs;
use tower_http::services::ServeDir;
use constcat::concat;

const ASSETS_PATH: &str = "assets/";
const INDEX_PATH: &str = concat!(ASSETS_PATH, "index.html");

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_index))
        .fallback_service(ServeDir::new("assets"));
    // Get the port from the environment variable, defaulting to 8000
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler to serve the index.html file
async fn serve_index() -> Html<String> {
    // Read the index.html file from the current directory
    match fs::read_to_string(INDEX_PATH).await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Error: index.html not found</h1>".to_string()),
    }
}
