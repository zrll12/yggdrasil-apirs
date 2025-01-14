use axum::Json;
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};

use crate::{AUTH_CONFIG, DATABASE, META_CONFIG};
use crate::controller::{ErrorResponse, ErrorResponses};
use crate::controller::auth_server::RATE_LIMIT_CACHE;
use crate::model::generated::prelude::User;
use crate::model::serialized::profile::SerializedProfile;
use crate::model::serialized::user::SerializedUser;
use crate::service::password::verify_password;
use crate::service::token::sign_new_token;

pub async fn authenticate(
    Json(request): Json<AuthenticateRequest>,
) -> Result<String, ErrorResponse> {
    let rate = RATE_LIMIT_CACHE.get(&request.username).await.unwrap_or(0);
    if rate > AUTH_CONFIG.login_rate_limit {
        return Err(ErrorResponses::InvalidCredentials.into());
    }
    RATE_LIMIT_CACHE.insert(request.username.clone(), rate + 1).await;
    
    let user = if request.username.contains("@") {
        User::find()
            .filter(crate::model::generated::user::Column::Email.eq(request.username))
            .one(&*DATABASE)
            .await
            .unwrap()
            .ok_or(ErrorResponses::InvalidCredentials)? 
    } else if META_CONFIG.feature.non_email_login { 
        User::find()
            .filter(crate::model::generated::user::Column::Username.eq(request.username))
            .one(&*DATABASE)
            .await
            .unwrap()
            .ok_or(ErrorResponses::InvalidCredentials)?
    } else { 
        return Err(ErrorResponses::InvalidCredentials.into());
    };

    if !verify_password(&request.password, &user.password) {
        return Err(ErrorResponses::InvalidCredentials.into());
    }

    let (access_token, client_token) = sign_new_token(user.id.clone(), request.client_token).await;

    let profiles: Vec<SerializedProfile> = crate::model::generated::profile::Entity::find()
        .filter(crate::model::generated::profile::Column::OwnerId.eq(user.id.clone()))
        .all(&*DATABASE)
        .await
        .unwrap()
        .into_iter()
        .map(|profile| SerializedProfile::from(profile.clone()))
        .collect();
    let selected_profile = profiles
        .iter()
        .find(|profile| profile.id == user.profile_id)
        .cloned();

    let user = SerializedUser::from(user);
    let response = AuthenticateResponse {
        access_token,
        client_token,
        available_profiles: profiles,
        selected_profile,
        user: if request.request_user == Some(true) {
            Some(user)
        } else {
            None
        },
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
    pub request_user: Option<bool>,
    pub client_token: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub client_token: String,
    pub available_profiles: Vec<SerializedProfile>,
    pub selected_profile: Option<SerializedProfile>,
    pub user: Option<SerializedUser>,
}
