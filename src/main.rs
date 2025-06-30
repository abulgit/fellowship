use axum::{
    routing::{get, post},
    http::Method,
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Serialize;
use std::sync::Arc;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod models;
mod crypto;
mod utils;

#[derive(Clone)]
struct AppState {
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {});

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/keypair", post(handlers::keypair::generate_keypair))
        .route("/token/create", post(handlers::token::create_token))
        .route("/token/mint", post(handlers::token::mint_token))
        .route("/message/sign", post(handlers::message::sign_message))
        .route("/message/verify", post(handlers::message::verify_message))
        .route("/send/sol", post(handlers::transfer::send_sol))
        .route("/send/token", post(handlers::transfer::send_token))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
                    
async fn health_check() -> impl IntoResponse {
    #[derive(Serialize)]
    struct HealthResponse {
        status: &'static str,
    }

    (StatusCode::OK, Json(HealthResponse { status: "ok" }))
}
