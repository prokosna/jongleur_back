use actix_web::*;
use app::end_user::{DetailedEndUserRepr, EndUserRepr};
use infra::rest::common::{CommonResponse, HttpStatus};

impl Responder for EndUserRepr {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

impl Responder for DetailedEndUserRepr {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}
