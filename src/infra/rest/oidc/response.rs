use actix_web::*;
use error_chain::ChainedError;

use domain::model::EndUserClaims;
use domain::service::{AuthorizeRet, AuthorizeRetKind, IntrospectRet, TokensRet, TokensRetKind};
use infra::rest::common::{CommonResponse, HttpStatus};

impl Responder for EndUserClaims {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

impl Responder for IntrospectRet {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

impl Responder for AuthorizeRet {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        match self.redirect_uri {
            Some(uri) => {
                match self.kind {
                    AuthorizeRetKind::Error { ref _cause, .. } => {
                        error!("{}", _cause.display_chain().to_string());
                    }
                    _ => {}
                }
                Ok(CommonResponse::redirect(self.kind, uri))
            }
            None => {
                let status = match self.kind {
                    AuthorizeRetKind::Error { ref _cause, .. } => {
                        error!("{}", _cause.display_chain().to_string());
                        let (status, _, _) = _cause.convert_status_content();
                        status
                    }
                    _ => HttpStatus::ok(),
                };
                Ok(CommonResponse::respond(self.kind, status))
            }
        }
    }
}

impl Responder for TokensRet {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        let status = match self.kind {
            TokensRetKind::Error { ref _cause, .. } => {
                error!("{}", _cause.display_chain().to_string());
                let (status, _, _) = _cause.convert_status_content();
                status
            }
            _ => HttpStatus::ok(),
        };
        Ok(CommonResponse::respond(self.kind, status))
    }
}
