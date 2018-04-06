use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};

use app::client::{ClientRepr, DetailedClientRepr};
use infra::rest::common::CommonResponse;

impl<'r> Responder<'r> for ClientRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

impl<'r> Responder<'r> for DetailedClientRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}
