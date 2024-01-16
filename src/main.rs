use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::VecDeque;
use axum::{
    routing::{get, post},
    http::{StatusCode},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any, AllowOrigin, AllowHeaders, AllowMethods};
use http::Method;
use std::result::Result;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Clone)]
struct BlogPost {
    id: usize,
    title: String,
    date: String,
    article: String,
}

#[derive(Serialize, Deserialize)]
struct NewBlogPost {
    title: String,
    article: String,
}

lazy_static! {
    static ref POSTS: Mutex<VecDeque<BlogPost>> = Mutex::new(VecDeque::new());
}

async fn create_post(Json(payload): Json<NewBlogPost>) -> Result<Json<BlogPost>, (StatusCode, String)> {
    if payload.title.is_empty() || payload.article.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title and article are required".to_string()));
    }

    let mut posts = POSTS.lock().unwrap();
    let id = posts.len();

    let new_post = BlogPost {
        id,
        title: payload.title,
        date: "2024-01-15".to_string(),
        article: payload.article,
    };

    posts.push_back(new_post.clone());

    Ok(Json(new_post))
}

async fn list_posts() -> Json<Vec<BlogPost>> {
    let posts = POSTS.lock().unwrap();
    Json(posts.iter().cloned().collect())
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(["http://localhost:5173".parse().unwrap(), "https://localhost:5001".parse().unwrap()]);

    let app = Router::new()
        .route("/posts", get(list_posts).post(create_post))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
