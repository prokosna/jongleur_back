//! Responders for domain errors
use error_chain::ChainedError;
use std::collections::HashMap;

use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use domain::error::domain as ed;
use infra::rest::common::response::HttpStatus;
use infra::rest::common::CommonResponse;

pub enum ResponseType {
    Undefined,
    Bearer,
}

impl ed::Error {
    pub fn convert_status_content(&self) -> (StatusCode, HashMap<String, String>, ResponseType) {
        let mut content = HashMap::new();
        content.insert("error".to_string(), self.description().to_string());
        content.insert("error_description".to_string(), self.to_string());
        match self {
            &ed::Error(ed::ErrorKind::RequireLogin(_), _) => {
                (HttpStatus::unauthorized(), content, ResponseType::Undefined)
            }
            &ed::Error(ed::ErrorKind::ServerError(_), _) => (
                HttpStatus::internal_server_error(),
                content,
                ResponseType::Undefined,
            ),
            &ed::Error(ed::ErrorKind::TemporarilyUnavailable(_), _) => (
                HttpStatus::service_unavailable(),
                content,
                ResponseType::Undefined,
            ),
            &ed::Error(ed::ErrorKind::UserinfoError(_), _) => {
                (HttpStatus::unauthorized(), content, ResponseType::Bearer)
            }
            _ => (HttpStatus::bad_request(), content, ResponseType::Undefined),
        }
    }

    pub fn respond(self) -> HttpResponse {
        let (status, content, _) = self.convert_status_content();
        CommonResponse::respond(content, status)
    }

    pub fn redirect(self, redirect_uri: String) -> HttpResponse {
        let (_, content, _) = self.convert_status_content();
        CommonResponse::redirect(content, redirect_uri)
    }

    pub fn bearer(self) -> HttpResponse {
        let (status, content, _) = self.convert_status_content();
        CommonResponse::bearer(content, status)
    }
}

// TODO: Dangerous workaround until error-chain 1.12.0
// https://github.com/rust-lang-nursery/error-chain/pull/241
unsafe impl Sync for ed::Error {}

impl ResponseError for ed::Error {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self.display_chain().to_string());
        let (status, content, response_type) = self.convert_status_content();
        match response_type {
            ResponseType::Bearer => CommonResponse::bearer(content, status),
            _ => CommonResponse::respond(content, status),
        }
    }
}
