use axum::routing::post;
use axum::Router;

use crate::controller::authserver::authenticate::authenticate;
use crate::controller::authserver::refresh::refresh;

mod authenticate;
mod refresh;

pub fn get_routers() -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/refresh", post(refresh))
}
