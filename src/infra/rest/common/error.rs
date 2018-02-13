//! Responders for domain errors
use rocket::response::{Responder, Response, ResponseBuilder};
use rocket::request::Request;
use rocket::http::Status;
use std::collections::HashMap;
use error_chain::ChainedError;

use domain::error::domain as ed;
use infra::rest::common::CommonResponse;

pub enum ResponseType {
    Undefined,
    Bearer,
}

impl ed::Error {
    pub fn convert_status_content(&self) -> (Status, HashMap<String, String>, ResponseType) {
        let mut content = HashMap::new();
        content.insert("error".to_string(), self.description().to_string());
        content.insert("error_description".to_string(), self.to_string());
        match self {
            &ed::Error(ed::ErrorKind::RequireLogin(_), _) => {
                (Status::Unauthorized, content, ResponseType::Undefined)
            }
            &ed::Error(ed::ErrorKind::ServerError(_), _) => (
                Status::InternalServerError,
                content,
                ResponseType::Undefined,
            ),
            &ed::Error(ed::ErrorKind::TemporarilyUnavailable(_), _) => {
                (Status::ServiceUnavailable, content, ResponseType::Undefined)
            }
            &ed::Error(ed::ErrorKind::UserinfoError(_), _) => {
                (Status::Unauthorized, content, ResponseType::Bearer)
            }
            _ => (Status::BadRequest, content, ResponseType::Undefined),
        }
    }

    pub fn respond<'r>(self) -> ResponseBuilder<'r> {
        let (status, content, _) = self.convert_status_content();
        CommonResponse::respond(content, status)
    }

    pub fn redirect<'r>(self, redirect_uri: String) -> ResponseBuilder<'r> {
        let (_, content, _) = self.convert_status_content();
        CommonResponse::redirect(content, redirect_uri)
    }

    pub fn bearer<'r>(self) -> ResponseBuilder<'r> {
        let (status, content, _) = self.convert_status_content();
        CommonResponse::bearer(content, status)
    }
}

impl<'r> Responder<'r> for ed::Error {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        error!("{}", self.display_chain().to_string());
        let (status, content, response_type) = self.convert_status_content();
        match response_type {
            ResponseType::Bearer => CommonResponse::bearer(content, status).ok(),
            _ => CommonResponse::respond(content, status).ok(),
        }
    }
}
