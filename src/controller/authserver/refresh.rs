use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::controller::{ErrorResponse, ErrorResponses};
use crate::model::generated::prelude::{Profile, User};
use crate::model::serialized::profile::SerializedProfile;
use crate::model::serialized::user::SerializedUser;
use crate::service::token::{
    check_token_state, get_token_info, invalidate_token, sign_new_token, TokenState,
};
use crate::DATABASE;

pub async fn refresh(Json(request): Json<RefreshRequest>) -> Result<String, ErrorResponse> {
    if check_token_state(&request.access_token, request.client_token.clone()).await == TokenState::Invalid {
        return Err(ErrorResponses::InvalidToken.into());
    }

    let token_info = get_token_info(&request.access_token).await.unwrap();
    let user = User::find()
        .filter(crate::model::generated::user::Column::Id.eq(&token_info.user_id))
        .one(&*DATABASE)
        .await
        .unwrap()
        .ok_or(ErrorResponses::InvalidToken)?;

    if let Some(profile) = request.selected_profile {
        let profile = Profile::find()
            .filter(crate::model::generated::profile::Column::Id.eq(profile.id))
            .one(&*DATABASE)
            .await
            .unwrap()
            .ok_or(ErrorResponses::InvalidProfile.to_error_response(None))?;

        if profile.owner_id != user.id{
            return Err(ErrorResponses::NoOwnership.into());
        }

        let mut user = user.clone().into_active_model();
        user.profile_id = Set(profile.id.clone());
        user.update(&*DATABASE).await.unwrap();
    }

    invalidate_token(&request.access_token).await;
    let (access_token, client_token) =
        sign_new_token(token_info.user_id.clone(), Some(token_info.client_token)).await;

    let response = RefreshResponse {
        access_token,
        client_token,
        selected_profile: None,
        user: Some(SerializedUser::from(user)),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub access_token: String,
    pub client_token: Option<String>,
    pub request_user: Option<bool>,
    pub selected_profile: Option<SerializedProfile>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub access_token: String,
    pub client_token: String,
    pub selected_profile: Option<SerializedProfile>,
    pub user: Option<SerializedUser>,
}
