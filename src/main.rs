use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = Router::new()
        .route("/", get(serve_homepage))
        .route("/blogs/{id}", get(serve_blog))
        .fallback(serve_static_page);

    // Run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Serve the homepage
async fn serve_homepage() -> impl IntoResponse {
    match fs::read_to_string("src/pages/index.html").await {
        Ok(content) => Html(content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Homepage not found").into_response(),
    }
}

// Serve blog posts
async fn serve_blog(axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    let blog_path = format!("src/pages/blogs/{}.html", id);
    
    match fs::read_to_string(&blog_path).await {
        Ok(content) => Html(content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Blog post not found").into_response(),
    }
}

// Fallback handler to serve other static pages
async fn serve_static_page(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path();
    
    // Remove leading slash and handle empty path
    let page_path = if path == "/" {
        "index.html"
    } else {
        &path[1..]
    };
    
    // Construct the full file path
    let full_path = format!("src/pages/{}", page_path);
    
    // Check if the file exists and is within the pages directory
    if Path::new(&full_path).exists() && full_path.starts_with("src/pages/") {
        match fs::read_to_string(&full_path).await {
            Ok(content) => Html(content).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "Page not found").into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Page not found").into_response()
    }
}