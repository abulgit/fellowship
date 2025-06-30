use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::{ApiResponse, SendSolRequest, SendTokenRequest, SolTransferResponse, TokenTransferAccount, InstructionResponse, Account};
use crate::utils::{self, generate_sol_transfer_instruction_data, generate_token_transfer_instruction_data};
use base64::{Engine as _, engine::general_purpose};
use base58::FromBase58;
use serde_json::json;

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> impl IntoResponse {
    if payload.from.is_empty() || payload.to.is_empty() {
        let response = ApiResponse::<SolTransferResponse> {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.lamports == 0 {
        let response = ApiResponse::<SolTransferResponse> {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.from.from_base58().is_err() {
        let response = ApiResponse::<SolTransferResponse> {
            success: false,
            data: None,
            error: Some("Invalid 'from' address format".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.to.from_base58().is_err() {
        let response = ApiResponse::<SolTransferResponse> {
            success: false,
            data: None,
            error: Some("Invalid 'to' address format".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    
    let accounts = vec![
        Account {
            pubkey: payload.from.clone(),
            is_signer: true,
            is_writable: true,
        },
        Account {
            pubkey: payload.to.clone(),
            is_signer: false,
            is_writable: true,
        },
    ];
    
    let instruction_data = generate_sol_transfer_instruction_data(payload.lamports);
    
    let sol_transfer = SolTransferResponse {
        program_id: utils::SYSTEM_PROGRAM_ID.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(instruction_data),
    };
    
    let response = ApiResponse {
        success: true,
        data: Some(sol_transfer),
        error: None,
    };
    
    (StatusCode::OK, Json(response))
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> impl IntoResponse {
    if payload.owner.is_empty() || payload.destination.is_empty() || payload.mint.is_empty() {
        let response = ApiResponse::<InstructionResponse> {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.amount == 0 {
        let response = ApiResponse::<InstructionResponse> {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    for (field, value) in [
        ("owner", &payload.owner),
        ("destination", &payload.destination),
        ("mint", &payload.mint),
    ] {
        if value.from_base58().is_err() {
            let response = ApiResponse::<InstructionResponse> {
                success: false,
                data: None,
                error: Some(format!("Invalid '{}' address format", field)),
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    }
    
    let accounts = vec![
        Account {
            pubkey: payload.owner.clone(),
            is_signer: true,
            is_writable: true,
        },
        Account {
            pubkey: payload.destination.clone(),
            is_signer: false,
            is_writable: true,
        },
          Account {
            pubkey: payload.mint.clone(),
            is_signer: false,
            is_writable: true,
        },
    ];
    
    let instruction_data = generate_token_transfer_instruction_data(payload.amount);
    
    let instruction = InstructionResponse {
        program_id: utils::SPL_TOKEN_PROGRAM_ID.to_string(),
        accounts,
        data: general_purpose::STANDARD.encode(instruction_data),
    };
                        
    let response = ApiResponse {
        success: true,
        data: Some(instruction),
        error: None,
    };
    
    (StatusCode::OK, Json(response))
} 