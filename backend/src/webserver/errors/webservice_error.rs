use crate::shared::application::errors::application_error::ApplicationError;
use crate::shared::domain::errors::domain_error::DomainError;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum WebserviceError {
    InternalServerError(String),
    BadRequest(String),
    NotFound(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl fmt::Display for WebserviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for WebserviceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            WebserviceError::InternalServerError(message) => HttpResponse::InternalServerError()
                .json(ErrorResponse {
                    code: 500,
                    error: "Internal Server Error".to_string(),
                    message: message.clone(),
                }),
            WebserviceError::BadRequest(message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    code: 400,
                    error: "Bad Request".to_string(),
                    message: message.clone(),
                })
            }
            WebserviceError::NotFound(message) => HttpResponse::NotFound().json(ErrorResponse {
                code: 404,
                error: "Not Found".to_string(),
                message: message.clone(),
            }),
            _ => HttpResponse::InternalServerError().json(ErrorResponse {
                code: 500,
                error: "Internal Server Error".to_string(),
                message: "Unexpected Error".to_string(),
            }),
        }
    }
}

impl From<DomainError> for WebserviceError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::TestError => WebserviceError::InternalServerError("TestError".to_string()),
            _ => WebserviceError::InternalServerError("Unexpected Error".to_string()),
        }
    }
}

impl From<ApplicationError> for WebserviceError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::TestError => {
                WebserviceError::InternalServerError("TestError".to_string())
            }
            _ => WebserviceError::InternalServerError("Unexpected Error".to_string()),
        }
    }
}

impl From<diesel::result::Error> for WebserviceError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            Error::NotFound => WebserviceError::NotFound(error.to_string()),
            Error::DatabaseError(_, _) => WebserviceError::BadRequest(error.to_string()),
            _ => WebserviceError::InternalServerError(error.to_string()),
        }
    }
}
