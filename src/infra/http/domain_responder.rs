use rocket::response::{self, Responder, Response, ResponseBuilder};
use rocket::request::Request;
use rocket::http::{ContentType, Status};
use serde_json;
use serde_urlencoded;
use std::io::Cursor;
use std::collections::HashMap;
use error_chain::ChainedError;

use application::oidc::{AuthResponse, AuthResponseKind, IntrospectResponse, TokensResponse};
use domain::error::domain as ed;
use domain::model::{Client, EndUser, EndUserClaims, Resource, Scope};
use infra::http::error_response::ErrorResponse;
use infra::http::client_response::{ClientResponse, RawClientResponse};
use infra::http::end_user_response::{EndUserResponse, RawEndUserResponse};
use infra::http::resource_response::{RawResourceResponse, ResourceResponse};

pub struct DomainParameters {
    redirect_uri: Option<String>,
    state: Option<String>,
}

impl DomainParameters {
    pub fn new(redirect_uri: Option<String>, state: Option<String>) -> Self {
        DomainParameters {
            redirect_uri,
            state,
        }
    }
}

enum DomainResponderKind {
    AuthDomain {
        ret: Result<AuthResponse, ed::Error>,
        params: DomainParameters,
    },
    TokensDomain {
        ret: Result<TokensResponse, ed::Error>,
    },
    Error { err: ed::Error },
    Client { client: Client },
    Clients { clients: Vec<Client> },
    RawClient { client: Client },
    Resource { resource: Resource },
    Resources { resources: Vec<Resource> },
    RawResource { resource: Resource },
    EndUser { end_user: EndUser },
    EndUsers { end_users: Vec<EndUser> },
    RawEndUser { end_user: EndUser },
    LoggedIn { id: String },
    RequireLogin { display_msg: Option<String> },
    InternalServerError { display_msg: Option<String> },
    Ok { display_msg: Option<String> },
    BadRequest { display_msg: Option<String> },
    Redirect { redirect_uri: String },
    Key { key: String },
    Userinfo { claims: EndUserClaims },
    Introspect { resp: IntrospectResponse },
}

pub struct DomainResponder {
    kind: DomainResponderKind,
}

impl DomainResponder {
    pub fn from_auth_domain(
        ret: Result<AuthResponse, ed::Error>,
        params: Option<DomainParameters>,
    ) -> Self {
        DomainResponder {
            kind: DomainResponderKind::AuthDomain {
                ret,
                params: params.unwrap_or(DomainParameters::new(None, None)),
            },
        }
    }

    pub fn from_tokens_domain(ret: Result<TokensResponse, ed::Error>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::TokensDomain { ret },
        }
    }

    pub fn from_domain_error(err: ed::Error) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Error { err },
        }
    }

    pub fn client(client: Client) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Client { client },
        }
    }

    pub fn clients(clients: Vec<Client>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Clients { clients },
        }
    }

    pub fn raw_client(client: Client) -> Self {
        DomainResponder {
            kind: DomainResponderKind::RawClient { client },
        }
    }

    pub fn resource(resource: Resource) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Resource { resource },
        }
    }

    pub fn resources(resources: Vec<Resource>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Resources { resources },
        }
    }

    pub fn raw_resource(resource: Resource) -> Self {
        DomainResponder {
            kind: DomainResponderKind::RawResource { resource },
        }
    }

    pub fn end_user(end_user: EndUser) -> Self {
        DomainResponder {
            kind: DomainResponderKind::EndUser { end_user },
        }
    }

    pub fn end_users(end_users: Vec<EndUser>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::EndUsers { end_users },
        }
    }

    pub fn raw_end_user(end_user: EndUser) -> Self {
        DomainResponder {
            kind: DomainResponderKind::RawEndUser { end_user },
        }
    }

    pub fn require_login(display_msg: Option<String>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::RequireLogin { display_msg },
        }
    }

    pub fn internal_server_error(display_msg: Option<String>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::InternalServerError { display_msg },
        }
    }

    pub fn logged_in(id: String) -> Self {
        DomainResponder {
            kind: DomainResponderKind::LoggedIn { id },
        }
    }

    pub fn ok(display_msg: Option<String>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Ok { display_msg },
        }
    }

    pub fn bad_request(display_msg: Option<String>) -> Self {
        DomainResponder {
            kind: DomainResponderKind::BadRequest { display_msg },
        }
    }

    pub fn redirect(redirect_uri: String) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Redirect { redirect_uri },
        }
    }

    pub fn key(key: String) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Key { key },
        }
    }

    pub fn userinfo(claims: EndUserClaims) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Userinfo { claims },
        }
    }

    pub fn introspect(resp: IntrospectResponse) -> Self {
        DomainResponder {
            kind: DomainResponderKind::Introspect { resp },
        }
    }

    fn domain_error(
        err: ed::Error,
        builder: ResponseBuilder,
        params: DomainParameters,
    ) -> ResponseBuilder {
        error!("{}", err.display_chain().to_string());
        if let Some(redirect_uri) = params.redirect_uri {
            // Error recirect
            // Authorization Code
            match err {
                ed::Error(ed::ErrorKind::RequireLogin, _) => {
                    let res = ErrorResponse::new(
                        err.description().to_string(),
                        Some(err.to_string()),
                        None,
                    );
                    res.respond(builder, Status::Unauthorized)
                }
                ed::Error(_, _) => {
                    let res = ErrorResponse::new(
                        err.description().to_string(),
                        Some(err.to_string()),
                        params.state.clone(),
                    );
                    res.redirect(builder, redirect_uri)
                }
            }
        } else {
            // Error response
            // Not Authorization Code
            let res = ErrorResponse::new(
                err.description().to_string(),
                Some(err.to_string()),
                params.state.clone(),
            );
            match err {
                ed::Error(ed::ErrorKind::ServerError(_), _) => {
                    res.respond(builder, Status::InternalServerError)
                }
                ed::Error(ed::ErrorKind::TemporarilyUnavailable(_), _) => {
                    res.respond(builder, Status::ServiceUnavailable)
                }
                ed::Error(ed::ErrorKind::RequireLogin, _) => {
                    res.respond(builder, Status::Unauthorized)
                }
                ed::Error(ed::ErrorKind::UserinfoError(_), _) => {
                    res.bearer_error(builder, Status::Unauthorized)
                }
                ed::Error(_, _) => res.respond(builder, Status::BadRequest),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequireAcceptance {
    status: String,
    code: String,
    grant_id: String,
    scope: Vec<Scope>,
}

impl<'r> Responder<'r> for DomainResponder {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let mut builder = Response::build();
        builder.header(ContentType::JSON);
        builder.raw_header("Cache-Control", "no-store");
        builder.raw_header("Pragma", "no-cache");
        match self.kind {
            DomainResponderKind::RequireLogin { display_msg } => {
                let mut content = HashMap::new();
                content.insert("status", "error".to_string());
                content.insert("code", "require_login".to_string());
                content.insert(
                    "display_msg",
                    display_msg.unwrap_or("Please log in.".to_string()),
                );
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                builder.status(Status::Unauthorized);
            }
            DomainResponderKind::InternalServerError { display_msg } => {
                let mut content = HashMap::new();
                content.insert("status", "error".to_string());
                content.insert("code", "server_error".to_string());
                content.insert(
                    "display_msg",
                    display_msg.unwrap_or("Unexpected error occurred.".to_string()),
                );
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                builder.status(Status::InternalServerError);
            }
            DomainResponderKind::LoggedIn { id } => {
                let mut content = HashMap::new();
                content.insert("status", "success".to_string());
                content.insert("id", id);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                builder.status(Status::Ok);
            }
            DomainResponderKind::Ok { display_msg } => {
                let mut content = HashMap::new();
                content.insert("status", "success".to_string());
                content.insert(
                    "display_msg",
                    display_msg.unwrap_or("Succeeded.".to_string()),
                );
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                builder.status(Status::Ok);
            }
            DomainResponderKind::BadRequest { display_msg } => {
                let mut content = HashMap::new();
                content.insert("status", "error".to_string());
                content.insert("code", "bad_request".to_string());
                content.insert(
                    "display_msg",
                    display_msg.unwrap_or("Bad request.".to_string()),
                );
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                builder.status(Status::BadRequest);
            }
            DomainResponderKind::Redirect { redirect_uri } => {
                let mut content = HashMap::new();
                content.insert("status", "redirect".to_string());
                content.insert("location", redirect_uri);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                builder.status(Status::Ok);
            }
            DomainResponderKind::Client { client } => {
                let content = ClientResponse::from_client(&client);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::Clients { clients } => {
                let content: Vec<ClientResponse> = clients
                    .iter()
                    .map(|c| ClientResponse::from_client(&c))
                    .collect();
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::RawClient { client } => {
                let content = RawClientResponse::from_client(&client);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::Resource { resource } => {
                let content = ResourceResponse::from_resource(&resource);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::Resources { resources } => {
                let content: Vec<ResourceResponse> = resources
                    .iter()
                    .map(|r| ResourceResponse::from_resource(&r))
                    .collect();
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::RawResource { resource } => {
                let content = RawResourceResponse::from_resource(&resource);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::EndUser { end_user } => {
                let content = EndUserResponse::from_end_user(&end_user);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::EndUsers { end_users } => {
                let content: Vec<EndUserResponse> = end_users
                    .iter()
                    .map(|u| EndUserResponse::from_end_user(&u))
                    .collect();
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::RawEndUser { end_user } => {
                let content = RawEndUserResponse::from_end_user(&end_user);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
            }
            DomainResponderKind::Error { err } => {
                builder =
                    DomainResponder::domain_error(err, builder, DomainParameters::new(None, None));
            }
            DomainResponderKind::AuthDomain { ret, params } => match ret {
                Ok(res) => match res.kind {
                    AuthResponseKind::RequireAcceptance { grant_id, scope } => {
                        let content = RequireAcceptance {
                            status: "requirement".to_string(),
                            code: "require_acceptance".to_string(),
                            grant_id,
                            scope,
                        };
                        builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                        builder.status(Status::Ok);
                    }
                    AuthResponseKind::Code { .. } => {
                        let qs = serde_urlencoded::to_string(&res.kind).unwrap();
                        let redirect_uri = format!("{}?{}", res.redirect_uri.unwrap(), &qs);
                        let mut content = HashMap::new();
                        content.insert("status", "redirect".to_string());
                        content.insert("location", redirect_uri);
                        builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                        builder.status(Status::Ok);
                    }
                    _ => {
                        let qs = serde_urlencoded::to_string(&res.kind).unwrap();
                        let redirect_uri = format!("{}#{}", res.redirect_uri.unwrap(), &qs);
                        let mut content = HashMap::new();
                        content.insert("status", "redirect".to_string());
                        content.insert("location", redirect_uri);
                        builder.sized_body(Cursor::new(serde_json::to_string(&content).unwrap()));
                        builder.status(Status::Ok);
                    }
                },
                Err(e) => {
                    builder = DomainResponder::domain_error(e, builder, params);
                }
            },
            DomainResponderKind::TokensDomain { ret } => match ret {
                Ok(res) => match res.kind {
                    _ => {
                        builder.status(Status::Ok);
                        builder.sized_body(Cursor::new(serde_json::to_string(&res.kind).unwrap()));
                    }
                },
                Err(e) => {
                    builder = DomainResponder::domain_error(
                        e,
                        builder,
                        DomainParameters::new(None, None),
                    );
                }
            },
            DomainResponderKind::Key { key } => {
                builder.header(ContentType::Plain);
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(key));
            }
            DomainResponderKind::Userinfo { claims } => {
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&claims).unwrap()));
            }
            DomainResponderKind::Introspect { resp } => {
                builder.status(Status::Ok);
                builder.sized_body(Cursor::new(serde_json::to_string(&resp).unwrap()));
            }
        }
        builder.ok()
    }
}
