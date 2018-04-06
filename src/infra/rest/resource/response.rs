use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};

use app::resource::{DetailedResourceRepr, ResourceRepr};
use infra::rest::common::CommonResponse;

impl<'r> Responder<'r> for ResourceRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

impl<'r> Responder<'r> for DetailedResourceRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}
