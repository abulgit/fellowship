use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::{ApiResponse, MessageSignRequest, MessageSignResponse, MessageVerifyRequest, MessageVerifyResponse};
use crate::crypto;

pub async fn sign_message(  
    Json(payload): Json<MessageSignRequest>,
) -> impl IntoResponse {
    if payload.message.is_empty() || payload.secret.is_empty() {
        let response = ApiResponse::<MessageSignResponse> {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    
    match crypto::sign_message(&payload.message, &payload.secret) {
        Ok((signature, pubkey)) => {
            let response = ApiResponse {
                success: true,
                data: Some(MessageSignResponse {
                    signature,
                    pubkey,
                    message: payload.message,
                }),
                error: None,
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse::<MessageSignResponse> {
                success: false,
                data: None,
                error: Some(format!("Error signing message: {}", e)),
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

pub async fn verify_message(
    Json(payload): Json<MessageVerifyRequest>,
) -> impl IntoResponse {
    if payload.message.is_empty() || payload.signature.is_empty() || payload.pubkey.is_empty() {
        let response = ApiResponse::<MessageVerifyResponse> {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    
    match crypto::verify_signature(&payload.message, &payload.signature, &payload.pubkey) {
        Ok(is_valid) => {
            let response = ApiResponse {
                success: true,
                data: Some(MessageVerifyResponse {
                    valid: is_valid,
                    message: payload.message,
                    pubkey: payload.pubkey.clone(),
                }),
                error: None,
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse::<MessageVerifyResponse> {
                success: false,
                data: None,
                error: Some(format!("Error verifying signature: {}", e)),
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
} 