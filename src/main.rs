use std::fmt::Debug;
use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::http;
use axum::http::{HeaderName, HeaderValue};
use axum_server::tls_rustls::RustlsConfig;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::net::SocketAddr;
use tower_http::classify::StatusInRangeAsFailures;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::log::{warn, LevelFilter};
use tracing::{debug, info, Level, Span};
use tracing_appender::non_blocking;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

use migration::{Migrator, MigratorTrait};

use crate::config::auth::AuthConfig;
use crate::config::core::CoreConfig;
use crate::config::get_config;
use crate::config::meta::MetaConfig;
use crate::config::texture::TextureConfig;

mod config;
mod controller;
mod model;
mod service;

lazy_static! {
    static ref CORE_CONFIG: CoreConfig = get_config("core");
    static ref AUTH_CONFIG: AuthConfig = get_config("auth");
    static ref TEXTURE_CONFIG: TextureConfig = get_config("textures");
    static ref META_CONFIG: MetaConfig = get_config("meta");
    static ref DATABASE: DatabaseConnection = {
        let mut opt = ConnectOptions::new(&CORE_CONFIG.db_uri);
        opt.sqlx_logging(true);
        opt.sqlx_logging_level(LevelFilter::Info);
        futures::executor::block_on(Database::connect(opt)).unwrap_or_else(|e| {
            panic!(
                "Failed to connect to database '{}': {}",
                CORE_CONFIG.db_uri, e
            )
        })
    };
}

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&CORE_CONFIG.trace_level));
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("log")
        .build("logs")
        .unwrap();
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    let formatting_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.f(%:z)".to_string()));
    let file_layer = fmt::layer()
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.f(%:z)".to_string()))
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();

    Migrator::up(&*DATABASE, None).await.unwrap();

    let trace_layer =
        TraceLayer::new(StatusInRangeAsFailures::new(400..=599).into_make_classifier())
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app = controller::all_routers()
        .layer(trace_layer)
        .layer(DefaultBodyLimit::max(
            CORE_CONFIG.max_body_size * 1024 * 1024,
        ))
        .layer(CorsLayer::permissive())
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_bytes(b"X-Authlib-Injector-API-Location").unwrap(),
            HeaderValue::from_static(&META_CONFIG.api_location),
        ));

    let addr = CORE_CONFIG.server_addr.parse().unwrap();
    info!("Listening: {addr}");

    if CORE_CONFIG.tls {
        debug!("HTTPS enabled.");
        let tls_config = RustlsConfig::from_pem_file(&CORE_CONFIG.ssl_cert, &CORE_CONFIG.ssl_key)
            .await
            .unwrap();
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    } else {
        warn!("HTTPS disabled.");
        axum_server::bind(addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }
}
