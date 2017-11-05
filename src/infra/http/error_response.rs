use rocket::response::ResponseBuilder;
use rocket::http::Status;
use serde_json;
use serde_urlencoded;
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")] error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: String, error_description: Option<String>, state: Option<String>) -> Self {
        ErrorResponse {
            error,
            error_description,
            state,
        }
    }

    pub fn respond(self, mut builder: ResponseBuilder, status: Status) -> ResponseBuilder {
        builder.status(status);
        builder.sized_body(Cursor::new(serde_json::to_string(&self).unwrap()));
        builder
    }

    pub fn redirect(self, mut builder: ResponseBuilder, redirect_uri: String) -> ResponseBuilder {
        // Jongleur is a SPA. So all requests are xhr,
        // Jongleur returns 200 and a location property instead of 302
        let qs = serde_urlencoded::to_string(&self).unwrap();
        let redirect_uri = format!("{}?{}", redirect_uri, &qs);
        let mut content = HashMap::new();
        content.insert("status", "redirect".to_string());
        content.insert("location", redirect_uri);
        builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
        builder.status(Status::Ok);
        builder
    }

    pub fn bearer_error(self, mut builder: ResponseBuilder, status: Status) -> ResponseBuilder {
        builder.status(status);
        let value = match self.error_description {
            Some(desc) => format!("error=\"{}\",error_description=\"{}\"", self.error, desc),
            None => format!("error=\"{}\"", self.error),
        };
        builder.raw_header("WWW-Authenticate", value);
        builder
    }
}
