use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::service::token::invalidate_token;

pub async fn invalidate(Json(request): Json<InvalidateRequest>) -> StatusCode {
    invalidate_token(&request.access_token).await;

    StatusCode::NO_CONTENT
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvalidateRequest {
    pub access_token: String,
    pub client_token: Option<String>,
}
