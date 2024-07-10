use axum::http::StatusCode;
use axum::Json;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use crate::controller::ErrorResponse;
use crate::service::token::invalidate_tokens;

pub async fn signout(Json(request): Json<SignoutRequest>) -> Result<StatusCode, ErrorResponse> {
    let user = crate::model::generated::prelude::User::find()
        .filter(crate::model::generated::user::Column::Email.eq(request.username.clone()))
        .one(&*crate::DATABASE)
        .await
        .unwrap()
        .ok_or(crate::controller::ErrorResponses::InvalidCredentials)?;
    invalidate_tokens(&user.id, 0).await;
    
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignoutRequest {
    pub username: String,
    pub password: String,
}