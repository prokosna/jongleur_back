//! Authorization Header Parser for Rocket
use base64::decode;
use regex::{Captures, Regex};
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use std::str;

pub enum AuthorizationType {
    Basic,
    Bearer,
    Undefined,
}

impl AuthorizationType {
    fn new(auth_type: &str) -> Self {
        let lower = auth_type.to_string().to_lowercase();
        match lower.as_str() {
            "basic" => AuthorizationType::Basic,
            "bearer" => AuthorizationType::Bearer,
            _ => AuthorizationType::Undefined,
        }
    }
}

pub struct AuthorizationHeader {
    pub auth_type: AuthorizationType,
    pub token: Option<String>,
}

impl AuthorizationHeader {
    fn new(auth_type: AuthorizationType, token: Option<String>) -> Self {
        AuthorizationHeader { auth_type, token }
    }

    pub fn get_basic_name_and_password(&self) -> Option<(String, String)> {
        match self.auth_type {
            AuthorizationType::Basic => self.token
                .as_ref()
                .and_then(|token| decode(&token).ok())
                .and_then(|decoded| {
                    str::from_utf8(decoded.as_slice())
                        .map(|x| x.to_string())
                        .ok()
                })
                .and_then(|userpass| {
                    let v = userpass
                        .split(":")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    if v.len() < 2 {
                        None
                    } else {
                        Some(v)
                    }
                })
                .map(|v| (v[0].clone(), v[1].clone())),
            _ => None,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizationHeader {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Parse Authorization header like below
        // Authorization: Basic name:pasword@base64
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*) (.*?)$").unwrap();
        }
        let basic_auth: Vec<_> = request.headers().get("authorization").collect();
        if basic_auth.len() != 1 {
            return Outcome::Success(AuthorizationHeader::new(AuthorizationType::Undefined, None));
        }
        let value = basic_auth[0].to_string();
        let caps: Captures = RE.captures(&value).unwrap();
        if caps.len() != 3 {
            return Outcome::Success(AuthorizationHeader::new(AuthorizationType::Undefined, None));
        }

        let auth_type = caps.get(1).map_or("", |x| x.as_str());
        let auth_type = AuthorizationType::new(auth_type);
        let token = caps.get(2).map(|x| x.as_str().to_string());
        Outcome::Success(AuthorizationHeader::new(auth_type, token))
    }
}
