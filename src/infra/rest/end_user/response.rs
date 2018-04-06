use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};

use app::end_user::{DetailedEndUserRepr, EndUserRepr};
use infra::rest::common::CommonResponse;

impl<'r> Responder<'r> for EndUserRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

impl<'r> Responder<'r> for DetailedEndUserRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}
