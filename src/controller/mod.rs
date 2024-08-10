use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use lazy_static::lazy_static;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use serde::Serialize;
use serde_json::{Map, Value};
use shadow_rs::shadow;
use crate::{META_CONFIG, TEXTURE_CONFIG};
use crate::service::crypto::SIGNATURE_KEY_PAIR;

mod api;
mod auth_server;
mod session_server;

lazy_static! {
    static ref PING_META: String = {
        shadow!(build);

        let version = format!("{}-{}", build::SHORT_COMMIT, build::BUILD_RUST_CHANNEL);
        let mut meta = Map::new();

        meta.insert("serverName".to_string(), Value::String(META_CONFIG.server_name.clone()));
        meta.insert("implementationName".to_string(), Value::String("Rust api implementation".to_string()));
        meta.insert("implementationVersion".to_string(), Value::String(version));



        if META_CONFIG.feature.non_email_login {
            meta.insert("feature.non_email_login".to_string(), Value::Bool(true));
        }

        let meta = PingMeta {
            meta: Value::Object(meta),
            skin_domains: TEXTURE_CONFIG.skin_domains.clone(),
            signature_publickey: SIGNATURE_KEY_PAIR
                .1
                .to_public_key_pem(LineEnding::LF)
                .unwrap()
                .to_string(),
        };

        serde_json::to_string(&meta).unwrap()
    };
}

pub fn all_routers() -> Router {
    Router::new()
        .route("/", get(ping))
        .route("/textures/:texture_id", get(api::texture::get_texture))
        .nest("/api", api::get_routers())
        .nest("/authserver", auth_server::get_routers())
        .nest("/sessionserver/session", session_server::get_routers())
}

pub async fn ping() -> &'static str {
    &PING_META
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PingMeta {
    pub meta: Value,
    pub skin_domains: Vec<String>,
    pub signature_publickey: String,
}

pub enum ErrorResponses {
    InvalidToken,       //令牌无效
    InvalidCredentials, //密码错误，或短时间内多次登录失败而被暂时禁止登录
    AlreadyBind,        //试图向一个已经绑定了角色的令牌指定其要绑定的角色
    NoOwnership,        //试图向一个令牌绑定不属于其对应用户的角色 （非标准）
    InvalidProfile,     //试图使用一个错误的角色加入服务器
}

impl ErrorResponses {
    pub fn to_error_response(&self, cause: Option<String>) -> ErrorResponse {
        match self {
            ErrorResponses::InvalidToken => ErrorResponse {
                http_code: StatusCode::FORBIDDEN,
                error: "ForbiddenOperationException".to_string(),
                error_message: "Invalid token.".to_string(),
                cause,
            },
            ErrorResponses::InvalidCredentials => ErrorResponse {
                http_code: StatusCode::FORBIDDEN,
                error: "ForbiddenOperationException".to_string(),
                error_message: "Invalid credentials. Invalid username or password.".to_string(),
                cause,
            },
            ErrorResponses::AlreadyBind => ErrorResponse {
                http_code: StatusCode::BAD_REQUEST,
                error: "IllegalArgumentException".to_string(),
                error_message: "Access token already has a profile assigned.".to_string(),
                cause,
            },
            ErrorResponses::NoOwnership => ErrorResponse {
                http_code: StatusCode::FORBIDDEN,
                error: "ForbiddenOperationException".to_string(),
                error_message: "".to_string(),
                cause,
            },
            ErrorResponses::InvalidProfile => ErrorResponse {
                http_code: StatusCode::FORBIDDEN,
                error: "ForbiddenOperationException".to_string(),
                error_message: "Invalid token.".to_string(),
                cause,
            },
        }
    }
}

pub struct ErrorResponse {
    pub http_code: StatusCode,
    pub error: String,
    pub error_message: String,
    pub cause: Option<String>,
}

impl ErrorResponse {
    fn with_cause(self, cause: Option<String>) -> ErrorResponse {
        ErrorResponse { cause, ..self }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": self.error,
            "errorMessage": self.error_message,
            "cause": self.cause,
        });
        (self.http_code, body.to_string()).into_response()
    }
}

impl From<ErrorResponses> for ErrorResponse {
    fn from(value: ErrorResponses) -> Self {
        value.to_error_response(None)
    }
}
