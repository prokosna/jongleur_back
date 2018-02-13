use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Status;

use app::admin::AdminRepr;
use infra::rest::common::CommonResponse;

impl<'r> Responder<'r> for AdminRepr {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}
