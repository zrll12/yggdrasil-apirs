mod create;
mod password;

use axum::Router;
use axum::routing::{get, MethodRouter, post};
use crate::controller::user::create::create_user;

pub fn get_routers() -> Router {
    Router::new()
         .route("/", post(create_user))
}