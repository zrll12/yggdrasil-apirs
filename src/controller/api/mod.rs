mod create;

use axum::Router;
use axum::routing::post;
use crate::controller::api::create::create_user;

pub fn get_routers() -> Router {
    Router::new()
         .route("/user", post(create_user))
}