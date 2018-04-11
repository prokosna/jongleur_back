use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use app::client::{ClientRepr, DetailedClientRepr};
use infra::rest::common::{CommonResponse, HttpStatus};

impl Responder for ClientRepr {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

impl Responder for DetailedClientRepr {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}
