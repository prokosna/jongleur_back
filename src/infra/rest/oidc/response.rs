use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Status;
use error_chain::ChainedError;

use domain::model::EndUserClaims;
use domain::service::{AuthorizeRet, AuthorizeRetKind, IntrospectRet, TokensRet, TokensRetKind};
use infra::rest::common::CommonResponse;

impl<'r> Responder<'r> for EndUserClaims {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

impl<'r> Responder<'r> for IntrospectRet {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

impl<'r> Responder<'r> for AuthorizeRet {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        match self.redirect_uri {
            Some(uri) => {
                match self.kind {
                    AuthorizeRetKind::Error { ref _cause, .. } => {
                        error!("{}", _cause.display_chain().to_string());
                    }
                    _ => {}
                }
                CommonResponse::redirect(self.kind, uri).ok()
            }
            None => {
                let status = match self.kind {
                    AuthorizeRetKind::Error { ref _cause, .. } => {
                        error!("{}", _cause.display_chain().to_string());
                        let (status, _, _) = _cause.convert_status_content();
                        status
                    }
                    _ => Status::Ok,
                };
                CommonResponse::respond(self.kind, status).ok()
            }
        }
    }
}

impl<'r> Responder<'r> for TokensRet {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        let status = match self.kind {
            TokensRetKind::Error { ref _cause, .. } => {
                error!("{}", _cause.display_chain().to_string());
                let (status, _, _) = _cause.convert_status_content();
                status
            }
            _ => Status::Ok,
        };
        CommonResponse::respond(self.kind, status).ok()
    }
}
