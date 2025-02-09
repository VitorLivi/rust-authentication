use actix_web::HttpResponse;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

pub type WebserviceResponse<T = HttpResponse> = Result<T, WebserviceError>;
