use axum::routing::{get, post};
use axum::Router;

mod join;
mod profile;

pub fn get_routers() -> Router {
    Router::new()
        .route("/minecraft/join", post(join::join_server))
        .route("/minecraft/hasJoined", get(join::has_joined_server))
        .route("/minecraft/profile/:profile_id", get(profile::get_profile))
}
