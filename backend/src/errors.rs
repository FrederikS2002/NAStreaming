use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiErrrorType {
    DBError,
    NotFoundError,
    InvalidInput,
    WriteError,
    ReadError,
    PermissionDeniedError,
}

#[derive(Debug)]
pub struct ApiError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub err_type: ApiErrrorType,
}

impl ApiError {
    fn message(&self) -> String {
        match &*self {
            ApiError {
                message: Some(message),
                cause: _,
                err_type: _,
            } => message.clone(),
            ApiError {
                message: None,
                cause: _,
                err_type: ApiErrrorType::NotFoundError,
            } => "Page not found".to_string(),
            _ => "An unexpected error occurred".to_string(),
        }
    }

    pub fn db_error(err: impl ToString) -> ApiError {
        ApiError {
            message: None,
            cause: Some(err.to_string()),
            err_type: ApiErrrorType::DBError,
        }
    }

    pub fn read_error(err: impl ToString) -> ApiError {
        ApiError {
            message: None,
            cause: Some(err.to_string()),
            err_type: ApiErrrorType::ReadError,
        }
    }

    pub fn write_error(err: impl ToString) -> ApiError {
        ApiError {
            message: None,
            cause: Some(err.to_string()),
            err_type: ApiErrrorType::WriteError,
        }
    }

    pub fn invalid_input_error(message: String) -> ApiError {
        ApiError {
            message: Some(message.to_string()),
            cause: None,
            err_type: ApiErrrorType::InvalidInput,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}
#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub cause: Option<String>
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            ApiErrrorType::DBError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrrorType::NotFoundError => StatusCode::NOT_FOUND,
            ApiErrrorType::InvalidInput => StatusCode::BAD_REQUEST,
            ApiErrrorType::WriteError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrrorType::ReadError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrrorType::PermissionDeniedError => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiErrorResponse {
            error: self.message(),
            cause: self.cause.clone(),
        })
    }
}
