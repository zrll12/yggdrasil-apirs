use axum::http::StatusCode;
use axum::Json;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use crate::controller::{ErrorResponse, ErrorResponses};
use crate::service::password::verify_password;
use crate::service::token::invalidate_tokens;

pub async fn signout(Json(request): Json<SignoutRequest>) -> Result<StatusCode, ErrorResponse> {
    let user = crate::model::generated::prelude::User::find()
        .filter(crate::model::generated::user::Column::Email.eq(request.username))
        .one(&*crate::DATABASE)
        .await
        .unwrap()
        .ok_or(ErrorResponses::InvalidCredentials)?;
    if !verify_password(&request.password, &user.password) {
        return Err(ErrorResponses::InvalidCredentials.into())
    }
    invalidate_tokens(&user.id, 0).await;
    
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignoutRequest {
    pub username: String,
    pub password: String,
}