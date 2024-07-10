use std::net::SocketAddr;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing::debug;
use crate::controller::{ErrorResponse, ErrorResponses};
use crate::DATABASE;
use crate::model::generated::prelude::User;
use crate::service::session::{save_session, SessionInfo};
use crate::service::token::{check_token_state, get_token_info};
use crate::service::token::TokenState::Valid;

pub async fn join_server(ConnectInfo(addr): ConnectInfo<SocketAddr>, Json(request): Json<JoinRequest>) -> Result<StatusCode, ErrorResponse> {
    if check_token_state(&request.access_token, None).await != Valid {
        return Err(ErrorResponses::InvalidToken.into());
    }
    let token_info = get_token_info(&request.access_token).await.unwrap();
    let user = User::find()
        .filter(crate::model::generated::user::Column::Id.eq(&token_info.user_id))
        .one(&*DATABASE)
        .await
        .unwrap()
        .ok_or(ErrorResponses::InvalidToken)?;
    
    if user.profile_id != request.selected_profile { 
        return Err(ErrorResponses::AlreadyBind.into())
    }
    
    debug!("Player {} joined the server at {}.", user.id, addr.to_string());
    
    let session_info = SessionInfo {
        access_token: request.access_token,
        client_ip: addr.to_string(),
    };
    save_session(request.server_id, session_info).await;
    
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequest {
    pub access_token: String,
    pub selected_profile: String,
    pub server_id: String,
}