use actix_web::middleware::Middleware;
use actix_web::middleware::Started;
use actix_web::Result;
use actix_web::{HttpMessage, HttpRequest};
use base64::decode;
use regex::Regex;
use std::str;

pub struct Authorization;

#[derive(Clone)]
pub enum AuthorizationType {
    Undefined,
    Basic { name: String, password: String },
    Bearer { token: String },
}

fn get_basic_name_and_password(value: &str) -> Option<(String, String)> {
    decode(value)
        .ok()
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
        .map(|v| (v[0].to_string(), v[1].to_string()))
}

impl<S> Middleware<S> for Authorization {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        let h = req.headers().clone();
        if let Some(auth_header) = h.get("authorization") {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(.*) (.*?)$").unwrap();
            }
            let caps = RE.captures(auth_header.to_str().unwrap_or("")).unwrap();
            if caps.len() != 3 {
                return Ok(Started::Done);
            }
            let auth_type = caps.get(1).map_or("", |x| x.as_str());
            let value = caps.get(2).map_or("", |x| x.as_str());
            match auth_type.to_string().to_lowercase().as_str() {
                "bearer" => {
                    let t = AuthorizationType::Bearer {
                        token: value.to_string(),
                    };
                    req.extensions().insert(t);
                    Ok(Started::Done)
                }
                "basic" => {
                    if let Some((name, password)) = get_basic_name_and_password(value) {
                        let t = AuthorizationType::Basic { name, password };
                        req.extensions().insert(t);
                        Ok(Started::Done)
                    } else {
                        req.extensions().insert(AuthorizationType::Undefined);
                        Ok(Started::Done)
                    }
                }
                _ => {
                    req.extensions().insert(AuthorizationType::Undefined);
                    Ok(Started::Done)
                }
            }
        } else {
            Ok(Started::Done)
        }
    }
}
