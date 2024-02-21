use axum::{
    http::StatusCode, routing::get, response::IntoResponse, Json, Router
};
use serde::Serialize;
use tokio;

#[tokio::main]
async fn main() {

    // build app with router
    let app = Router::new()
        .route("/api/v1/health", get(health_check));

    // run
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
#[derive(Serialize)]
struct HealthCheckResponse {
    status: String
}

async fn health_check() -> impl IntoResponse {

    let resp = HealthCheckResponse {status: "ok".to_string()};

   (StatusCode::OK, Json(resp))
}

// basic handler that responds with a static string
#[derive(Serialize)]
struct HealthCheckResponse {
    status: String
}
