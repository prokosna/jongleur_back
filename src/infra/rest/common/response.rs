//! Common responders for Rocket
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response, ResponseBuilder};
use serde::Serialize;
use serde_json;
use serde_urlencoded;
use std::collections::HashMap;
use std::io::Cursor;

pub struct CommonResponse {}

impl CommonResponse {
    pub fn respond<'r, T: Serialize>(model: T, status: Status) -> ResponseBuilder<'r> {
        let mut builder = Response::build();
        builder.header(ContentType::JSON);
        builder.raw_header("Cache-Control", "no-store");
        builder.raw_header("Pragma", "no-cache");
        builder.sized_body(Cursor::new(serde_json::to_string(&model).unwrap()));
        builder.status(status);
        builder
    }

    pub fn redirect<'r, T: Serialize>(model: T, redirect_uri: String) -> ResponseBuilder<'r> {
        // Jongleur is a SPA. So all requests are xhr,
        // Jongleur returns 200 and a location property instead of 302
        let mut builder = Response::build();
        let qs = serde_urlencoded::to_string(&model).unwrap();
        let redirect_uri = format!("{}?{}", redirect_uri, &qs);
        let mut content = HashMap::new();
        content.insert("status", "redirect".to_string());
        content.insert("location", redirect_uri);
        builder.header(ContentType::JSON);
        builder.raw_header("Cache-Control", "no-store");
        builder.raw_header("Pragma", "no-cache");
        builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
        builder.status(Status::Ok);
        builder
    }

    pub fn bearer<'r, T: Serialize>(model: T, status: Status) -> ResponseBuilder<'r> {
        let mut builder = Response::build();
        let qs = serde_urlencoded::to_string(&model).unwrap();
        let value = qs.replace("&", ",");
        builder.raw_header("Cache-Control", "no-store");
        builder.raw_header("Pragma", "no-cache");
        builder.raw_header("WWW-Authenticate", value);
        builder.status(status);
        builder
    }
}

pub struct CommonListResponse<T: Serialize> {
    pub list: Vec<T>,
}

impl<'r, T: Serialize> Responder<'r> for CommonListResponse<T> {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(self.list, Status::Ok).ok()
    }
}
