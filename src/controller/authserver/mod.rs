mod authenticate;

use axum::Router;
use axum::routing::{get, post};
use crate::controller::authserver::authenticate::authenticate;

pub fn get_routers() -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
}