use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::crypto;
use crate::models::{ApiResponse, KeypairResponse};

pub async fn generate_keypair() -> impl IntoResponse {
    let (pubkey, secret) = crypto::generate_keypair();

    let response = ApiResponse {
        success: true,
        data: Some(KeypairResponse {
            pubkey,
            secret,
        }),
        error: None,
    };
    (StatusCode::OK, Json(response))
} 