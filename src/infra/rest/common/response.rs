//! Common responders for Actix-web
use actix_web::http::StatusCode;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde::Serialize;
use serde_urlencoded;
use std::collections::HashMap;

pub struct CommonResponse {}

pub struct HttpStatus {}

impl HttpStatus {
    pub fn ok() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(200u16).unwrap();
        }
        CODE.clone()
    }

    pub fn found() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(302u16).unwrap();
        }
        CODE.clone()
    }

    pub fn bad_request() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(400u16).unwrap();
        }
        CODE.clone()
    }

    pub fn unauthorized() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(401u16).unwrap();
        }
        CODE.clone()
    }

    pub fn not_found() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(404u16).unwrap();
        }
        CODE.clone()
    }

    pub fn method_not_allowed() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(405u16).unwrap();
        }
        CODE.clone()
    }

    pub fn internal_server_error() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(500u16).unwrap();
        }
        CODE.clone()
    }

    pub fn service_unavailable() -> StatusCode {
        lazy_static! {
            static ref CODE: StatusCode = StatusCode::from_u16(503u16).unwrap();
        }
        CODE.clone()
    }
}

impl CommonResponse {
    pub fn respond<T: Serialize>(model: T, status: StatusCode) -> HttpResponse {
        let mut builder = HttpResponse::build(status);
        builder
            .header("Cache-Control", "no-store")
            .header("Pragma", "no-cache")
            .json(&model)
    }

    pub fn redirect<T: Serialize>(model: T, redirect_uri: String) -> HttpResponse {
        // Jongleur is a SPA. So all requests are xhr,
        // Jongleur returns 200 and a location property instead of 302
        let mut builder = HttpResponse::build(HttpStatus::found());
        let qs = serde_urlencoded::to_string(&model).unwrap();
        let redirect_uri = format!("{}?{}", redirect_uri, &qs);
        let mut content = HashMap::new();
        content.insert("status", "redirect".to_string());
        content.insert("location", redirect_uri);
        builder
            .header("Cache-Control", "no-store")
            .header("Pragma", "no-cache")
            .json(&content)
    }

    pub fn bearer<T: Serialize>(model: T, status: StatusCode) -> HttpResponse {
        let mut builder = HttpResponse::build(status);
        let qs = serde_urlencoded::to_string(&model).unwrap();
        let value = qs.replace("&", ",");
        builder
            .header("Cache-Control", "no-store")
            .header("Pragma", "no-cache")
            .header("WWW-Authenticate", value)
            .finish()
    }
}

pub struct CommonListResponse<T: Serialize> {
    pub list: Vec<T>,
}

impl<T: Serialize> Responder for CommonListResponse<T> {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(self.list, HttpStatus::ok()))
    }
}
