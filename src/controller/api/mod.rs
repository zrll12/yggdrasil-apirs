mod create;
mod texture;

use axum::Router;
use axum::routing::{post, put};

pub fn get_routers() -> Router {
    Router::new()
        .route("/user", post(create::create_user))
        .route("/user/profile/:uuid/:type", put(texture::upload_texture))
}