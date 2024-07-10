use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::controller::ErrorResponse;
use crate::controller::ErrorResponses::InvalidToken;
use crate::service::token::{check_token_state, TokenState};

pub async fn validate(Json(request): Json<ValidateRequest>) -> Result<StatusCode, ErrorResponse>{
    if check_token_state(&request.access_token, request.client_token.clone()).await != TokenState::Valid {
        return Err(InvalidToken.into());
    }
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValidateRequest{
    pub access_token: String,
    pub client_token: Option<String>,
}