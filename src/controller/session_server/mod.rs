use axum::routing::{get, post};
use axum::Router;

use crate::controller::session_server::join::join_server;
use crate::controller::session_server::profile::get_profile;

mod join;
mod profile;

pub fn get_routers() -> Router {
    Router::new()
        .route("/minecraft/join", post(join_server))
        .route("/minecraft/profile/:profile_id", get(get_profile))
}
