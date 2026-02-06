use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::fs;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(serve_index));

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
    match fs::read_to_string("index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Error: index.html not found</h1>".to_string()),
    }
}