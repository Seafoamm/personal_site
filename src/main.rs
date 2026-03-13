use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

mod templates;
use templates::{handler_algorithms, handler_index, handler_system_design};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_index))
        .route("/about", get(handler_index))
        .route("/system_design", get(handler_system_design))
        .route("/algorithms", get(handler_algorithms))
        .fallback_service(ServeDir::new("assets"))
        // Add this layer last so it wraps all routes
        .layer(LiveReloadLayer::new());

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
