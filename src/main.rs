#![feature(plugin, custom_derive)]
#![recursion_limit = "1024"]
#![allow(dead_code)]

extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate blake2;
#[macro_use(bson, doc)]
extern crate bson;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate http;
extern crate jsonwebtoken;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mongo_driver;
extern crate mongodb;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate redis;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate time;
extern crate url;

mod app;
mod config;
mod constant;
mod domain;
mod infra;
mod server;
mod util;

use actix_web::http::Method;
use actix_web::*;
use app::initialize::{InitializeService, InitializeServiceComponent};
use infra::rest;
use infra::rest::common::HttpStatus;
use infra::rest::middleware::Authorization;
use server::ApplicationState;

fn initialize(state: &ApplicationState) {
    let initialize_service = state.server.initialize_service();
    initialize_service.initialize();
}

fn configure_cors(methods: Vec<Method>) -> middleware::cors::Cors {
    middleware::cors::Cors::build()
        .allowed_methods(methods)
        .finish()
}

fn main() {
    dotenv::from_filename("./Config.env").ok();
    actix_web::server::HttpServer::new(|| {
        let state = ApplicationState::new();
        initialize(&state);
        App::with_state(state)
            .resource("/admins", |r| {
                let cors = configure_cors(vec![Method::GET, Method::POST]);
                cors.register(r);
                r.method(http::Method::GET).with(rest::admin::get_admins);
                r.method(http::Method::POST)
                    .with2(rest::admin::register_admin)
            })
            .resource("/admins/{id}", |r| {
                r.middleware(Authorization {});
                r.method(http::Method::GET).with2(rest::admin::get_admin);
                r.method(http::Method::PUT).with3(rest::admin::update_admin);
                r.method(http::Method::DELETE)
                    .with2(rest::admin::delete_admin)
            })
            .resource("/admins/login", |r| {
                r.method(http::Method::POST).with2(rest::admin::login)
            })
            .resource("/admins/logout", |r| {
                r.middleware(Authorization {});
                r.method(http::Method::POST).with(rest::admin::logout)
            })
            .resource("/clients", |r| {
                let cors = configure_cors(vec![Method::GET, Method::POST]);
                cors.register(r);
                r.method(Method::GET).with2(rest::client::get_clients);
                r.method(Method::POST).with2(rest::client::register_client)
            })
            .resource("/clients/{id}", |r| {
                let cors = configure_cors(vec![Method::GET]);
                cors.register(r);
                r.middleware(Authorization {});
                r.method(Method::GET).with2(rest::client::get_client);
                r.method(Method::PUT).with3(rest::client::update_client);
                r.method(Method::DELETE).with2(rest::client::delete_client)
            })
            .resource("/clients/{id}/detail", |r| {
                r.middleware(Authorization {});
                r.method(Method::GET)
                    .with2(rest::client::get_detailed_client)
            })
            .resource("/clients/login", |r| {
                r.method(http::Method::POST).with2(rest::client::login)
            })
            .resource("/clients/logout", |r| {
                r.middleware(Authorization {});
                r.method(http::Method::POST).with(rest::client::logout)
            })
            .resource("/end_users", |r| {
                let cors = configure_cors(vec![Method::GET, Method::POST]);
                cors.register(r);
                r.method(Method::GET).with(rest::end_user::get_end_users);
                r.method(Method::POST)
                    .with2(rest::end_user::register_end_user)
            })
            .resource("/end_users/{id}", |r| {
                let cors = configure_cors(vec![Method::GET]);
                cors.register(r);
                r.middleware(Authorization {});
                r.method(Method::GET).with2(rest::end_user::get_end_user);
                r.method(Method::PUT).with3(rest::end_user::update_end_user);
                r.method(Method::DELETE)
                    .with2(rest::end_user::delete_end_user)
            })
            .resource("/end_users/{id}/detail", |r| {
                r.middleware(Authorization {});
                r.method(Method::GET)
                    .with2(rest::end_user::get_detailed_end_user)
            })
            .resource("/end_users/login", |r| {
                r.method(Method::POST).with2(rest::end_user::login)
            })
            .resource("/end_users/logout", |r| {
                r.middleware(Authorization {});
                r.method(Method::POST).with(rest::end_user::logout)
            })
            .resource("/resources", |r| {
                let cors = configure_cors(vec![Method::GET, Method::POST]);
                cors.register(r);
                r.method(Method::GET).with(rest::resource::get_resources);
                r.method(Method::POST)
                    .with2(rest::resource::register_resource)
            })
            .resource("/resources/{id}", |r| {
                let cors = configure_cors(vec![Method::GET]);
                cors.register(r);
                r.middleware(Authorization {});
                r.method(Method::GET).with2(rest::resource::get_resource);
                r.method(Method::PUT).with3(rest::resource::update_resource);
                r.method(Method::DELETE)
                    .with2(rest::resource::delete_resource)
            })
            .resource("/resources/{id}/detail", |r| {
                r.middleware(Authorization {});
                r.method(Method::GET)
                    .with2(rest::resource::get_detailed_resource)
            })
            .resource("/resources/login", |r| {
                r.method(Method::POST).with2(rest::resource::login)
            })
            .resource("/resources/logout", |r| {
                r.middleware(Authorization {});
                r.method(Method::POST).with(rest::resource::logout)
            })
            .resource("/oidc/authorize", |r| {
                r.middleware(Authorization {});
                r.method(Method::GET).with2(rest::oidc::authorize)
            })
            .resource("/oidc/accept", |r| {
                r.middleware(Authorization {});
                r.method(Method::POST).with2(rest::oidc::accept_client)
            })
            .resource("/oidc/tokens", |r| {
                let cors = configure_cors(vec![Method::POST]);
                cors.register(r);
                r.middleware(Authorization {});
                r.method(Method::POST).with2(rest::oidc::get_tokens)
            })
            .resource("/oidc/introspect", |r| {
                let cors = configure_cors(vec![Method::POST]);
                cors.register(r);
                r.middleware(Authorization {});
                r.method(Method::POST).with2(rest::oidc::introspect)
            })
            .resource("/oidc/userinfo", |r| {
                let cors = configure_cors(vec![Method::GET]);
                cors.register(r);
                r.middleware(Authorization {});
                r.method(Method::POST).with(rest::oidc::get_userinfo)
            })
            .resource("/oidc/publickey", |r| {
                let cors = configure_cors(vec![Method::GET]);
                cors.register(r);
                r.method(Method::POST).with(rest::oidc::get_publickey)
            })
            .resource("/health", |r| {
                let cors = configure_cors(vec![Method::GET]);
                cors.register(r);
                r.method(http::Method::GET).f(rest::health::get_health)
            })
            .default_resource(|r| {
                r.method(Method::GET)
                    .f(|_req| HttpResponse::build(HttpStatus::not_found()).finish());
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::build(HttpStatus::method_not_allowed()).finish())
            })
    }).bind("127.0.0.1:8000")
        .unwrap()
        .run()
}
