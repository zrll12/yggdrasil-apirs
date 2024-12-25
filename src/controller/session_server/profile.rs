use axum::extract::{Path, Query};
use axum::http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::model::generated::prelude::Profile;
use crate::model::serialized::profile::SerializedProfile;

pub async fn get_profile(Path(profile_id): Path<String>, Query(query): Query<GetProfileRequestQuery>) -> Result<String, StatusCode> {
    let profile = Profile::find()
        .filter(crate::model::generated::profile::Column::Id.eq(profile_id))
        .one(&*crate::DATABASE)
        .await
        .unwrap()
        .ok_or(StatusCode::NO_CONTENT)?;

    let mut profile = SerializedProfile::from(profile);

    if !query.unsigned.unwrap_or(true) {
        profile.sign().await;
    }

    Ok(serde_json::to_string(&profile).unwrap())
}

#[derive(Deserialize, Serialize)]
pub struct GetProfileRequestQuery {
    unsigned: Option<bool>,
}