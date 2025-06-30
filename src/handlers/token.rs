use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::{ApiResponse, TokenCreateRequest, TokenMintRequest, InstructionResponse};
use crate::utils::{self, create_account, generate_token_create_instruction_data, generate_token_mint_instruction_data};
use base58::FromBase58;

pub async fn create_token(
    Json(payload): Json<TokenCreateRequest>,
) -> impl IntoResponse {
    if payload.mint.is_empty() || payload.mint_authority.is_empty() {
        let response = ApiResponse::<InstructionResponse> {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.decimals > 9 {
        let response = ApiResponse::<InstructionResponse> {
            success: false,
            data: None,
            error: Some("Decimals must be between 0 and 9".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.mint.from_base58().is_err() {
        let response = ApiResponse::<InstructionResponse> {
            success: false,
            data: None,
            error: Some("Invalid mint address format".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    if payload.mint_authority.from_base58().is_err() {
        let response = ApiResponse::<InstructionResponse> {
            success: false,
            data: None,
            error: Some("Invalid mint authority address format".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    
    let accounts = vec![
        create_account(&payload.mint, false, true),
        create_account(&payload.mint_authority, true, false),
    ];
    
    let instruction_data = generate_token_create_instruction_data(payload.decimals);
    
    let instruction = utils::generate_token_instruction(
        utils::SPL_TOKEN_PROGRAM_ID,
        accounts,
        instruction_data,
    );
    
    let response = ApiResponse {
        success: true,
        data: Some(instruction),
        error: None,
    };
    
    (StatusCode::OK, Json(response))
}

pub async fn mint_token(
    Json(payload): Json<TokenMintRequest>,
) -> impl IntoResponse {
    if payload.mint.is_empty() || payload.destination.is_empty() || payload.authority.is_empty() {
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
        ("mint", &payload.mint),
        ("destination", &payload.destination),
        ("authority", &payload.authority),
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
        create_account(&payload.mint, false, true),
        create_account(&payload.destination, false, true),
        create_account(&payload.authority, true, false),
    ];
    
    let instruction_data = generate_token_mint_instruction_data(payload.amount);
    
    let instruction = utils::generate_token_instruction(
        utils::SPL_TOKEN_PROGRAM_ID,
        accounts,
        instruction_data,
    );
                let response = ApiResponse {
        success: true,
        data: Some(instruction),
        error: None,
    };
    
    (StatusCode::OK, Json(response))
} 