use axum::Json;
use sea_orm::{ActiveModelTrait, NotSet};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::DATABASE;
use crate::model::serialized::uuid::UuidNoChar;
use crate::service::password::generate_password_hash;

pub async fn create_user(Json(request): Json<CreateUserRequest>) {
    let profile_id = UuidNoChar::new().to_string();
    let user_id = UuidNoChar::new().to_string();
    crate::model::generated::profile::ActiveModel {
        id: Set(profile_id.clone()),
        name: Set(request.name),
        model: NotSet,
        owner_id: Set(user_id.clone()),
        skin_texture: NotSet,
        cape_texture: NotSet,
        create_time: NotSet,
        update_time: NotSet,
    }.insert(&*DATABASE).await.unwrap();
    
    crate::model::generated::user::ActiveModel {
        id: Set(user_id),
        username: NotSet,
        email: Set(request.email),
        password: Set(generate_password_hash(&request.password)),
        profile_id: Set(profile_id),
        preferred_language: Set(request.preferred_language),
        create_time: NotSet,
        update_time: NotSet,
    }.insert(&*DATABASE).await.unwrap();
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub preferred_language: Option<String>,
}