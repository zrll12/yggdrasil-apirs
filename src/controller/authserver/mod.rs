use axum::routing::post;
use axum::Router;

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
