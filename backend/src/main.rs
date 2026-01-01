use axum::{Router, http::StatusCode, response::{IntoResponse, Json}, routing::{get, delete, post, patch}};
use serde::Serialize;
use serde_json::json;
use tokio::net::TcpListener;
use std::sync::{Arc, RwLock};

mod defs;
mod handlers;
mod db;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

#[derive(Serialize)]
#[derive(Debug)]
pub enum ApiError {
    NotFound, // 404 Not Found
    InvalidInput(String), // 400 Bad Request
    InternalError // 500 Internal Service Error
}
#[derive(Serialize)]
struct Health {
    status: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
       let (status, error_msg) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND, "Data not found".to_string(),
            ),
            ApiError::InvalidInput(msg) => (
                StatusCode::BAD_REQUEST, msg,
            ), 
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string(),
            )  
        };

        let body = Json(json!({"error": error_msg}));

        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();


    let app = create_app();

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("🚀 Server running on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();

}

async fn health_check() -> axum::Json<Health> {
    axum::Json(Health { status: "ok" })
}

fn create_app() -> Router {
    let def_repo = defs::definition_repository::DefinitionManager::new();
    let state = Arc::new(RwLock::new(def_repo));

    Router::new()
    .route("/health", get(health_check))
    .route("/definitions", get(handlers::list_defs))
    .route("/definitions", post(handlers::create_def))
    .route("/definitions/{id}", get(handlers::get_def_by_id))
    .route("/definitions/{id}", delete(handlers::remove_by_id))
    .route("/definitions/{id}", patch(handlers::update_def))
    .route("/defintions/search/{term}", get(handlers::get_def_by_term))
    .with_state(state)
}
