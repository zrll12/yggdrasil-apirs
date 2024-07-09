mod authenticate;

use axum::Router;
use axum::routing::{get, post};

pub fn get_routers() -> Router {
    Router::new()
        .route("/authenticate", get(||async { "Hello, World!"}))
}