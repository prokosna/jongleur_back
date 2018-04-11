use actix_web::*;
use app::resource::{DetailedResourceRepr, ResourceRepr};
use infra::rest::common::{CommonResponse, HttpStatus};

impl Responder for ResourceRepr {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

impl Responder for DetailedResourceRepr {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}
