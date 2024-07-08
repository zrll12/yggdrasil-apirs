mod user;

use axum::{Router};
use axum::routing::get;

pub fn all_routers() -> Router {
    Router::new()
        .nest("/user", user::get_routers())
}