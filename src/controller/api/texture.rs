use axum::extract::{Multipart, Path};
use axum::http::{HeaderMap, StatusCode};
use log::{debug, warn};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, IntoActiveModel, ActiveModelTrait};
use sea_orm::ActiveValue::Set;
use crate::model::generated::prelude::Profile;
use crate::service::file::write_file;
use crate::service::token::get_token_info;
use crate::TEXTURE_CONFIG;

pub async fn upload_texture(header_map: HeaderMap, Path((profile_id, texture_type)): Path<(String, String)>, mut multipart: Multipart) -> StatusCode {
    let token = if let Some(a) = header_map.get("Authorization") {
        a.to_str().unwrap()
    } else {
        return StatusCode::UNAUTHORIZED;
    };
    let token = token.replace("Bearer ", "");

    if !TEXTURE_CONFIG.allow_cape && texture_type == "cape" {
        return StatusCode::FORBIDDEN;
    }
    if !TEXTURE_CONFIG.allow_skin && texture_type == "skin" {
        return StatusCode::FORBIDDEN;
    }

    let token_info = if let Some(a) = get_token_info(&token).await {
        a
    } else {
        warn!("token: {}", token);
        return StatusCode::UNAUTHORIZED;
    };

    let profile = Profile::find()
        .filter(crate::model::generated::profile::Column::Id.eq(profile_id))
        .filter(crate::model::generated::profile::Column::OwnerId.eq(token_info.user_id))
        .one(&*crate::DATABASE)
        .await.unwrap();
    if profile.is_none() {
        return StatusCode::UNAUTHORIZED;
    }
    let profile = profile.unwrap();
    let mut file_id = String::new();
    let mut model_type = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_type = field.content_type().map(|a| a.to_string());
        let name = field.name().unwrap().to_string();
        let data = match field.bytes().await {
            Ok(a) => a,
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        match name.as_str() {
            "file" => {
                let Some(file_type) = file_type else { return StatusCode::BAD_REQUEST; };
                if file_type != "image/png" {
                    debug!("Invalid file type: {}", file_type);
                    return StatusCode::BAD_REQUEST;
                }

                let Some(id) = write_file(data).await else {return StatusCode::INTERNAL_SERVER_ERROR;};
                file_id = id;
            }
            "model" => {
                if texture_type != "skin" {
                    continue;
                }

                model_type = String::from_utf8(data.to_vec()).unwrap()
            }
            _ => {}
        }
    }

    if file_id == "" {
        return StatusCode::BAD_REQUEST;
    }

    let mut profile = profile.into_active_model();
    if texture_type == "skin" {
        profile.skin_texture = Set(Some(file_id));
        profile.model = Set(model_type);
    } else {
        profile.cape_texture = Set(Some(file_id));
    }
    profile.update(&*crate::DATABASE).await.unwrap();

    StatusCode::NO_CONTENT
}