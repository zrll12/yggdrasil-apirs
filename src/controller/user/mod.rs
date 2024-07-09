mod create;

use axum::Router;
use axum::routing::post;
use crate::controller::user::create::create_user;

pub fn get_routers() -> Router {
    Router::new()
         .route("/", post(create_user))
}