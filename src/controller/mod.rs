mod user;
mod authserver;

use axum::{Router};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

pub fn all_routers() -> Router {
    Router::new()
        .nest("/user", user::get_routers())
        .nest("/authserver", authserver::get_routers())
}

pub enum ErrorResponses {
    InvalidToken,//令牌无效
    InvalidCredentials,//密码错误，或短时间内多次登录失败而被暂时禁止登录
    AlreadyBind,//试图向一个已经绑定了角色的令牌指定其要绑定的角色
    NoOwnership,//试图向一个令牌绑定不属于其对应用户的角色 （非标准）
    InvalidProfile,//试图使用一个错误的角色加入服务器
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
        ErrorResponse {
            cause,
            ..self
        }
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