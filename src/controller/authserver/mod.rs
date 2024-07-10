use axum::routing::post;
use axum::Router;
use lazy_static::lazy_static;
use moka::future::Cache;

mod authenticate;
mod invalidate;
mod refresh;
mod validate;
mod signout;

pub fn get_routers() -> Router {
    Router::new()
        .route("/authenticate", post(authenticate::authenticate))
        .route("/refresh", post(refresh::refresh))
        .route("/validate", post(validate::validate))
        .route("/invalidate", post(invalidate::invalidate))
        .route("/signout", post(signout::signout))
}

lazy_static!{
    static ref RATE_LIMIT_CACHE: Cache<String, u32> = Cache::builder()
        .time_to_live(std::time::Duration::from_secs(60))
        .build();
}