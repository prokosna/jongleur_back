#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![recursion_limit = "1024"]
#![allow(dead_code)]

extern crate base64;
extern crate blake2;
#[macro_use(bson, doc)]
extern crate bson;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
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
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
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

use app::initialize::{InitializeService, InitializeServiceComponent};
use infra::rest;
use infra::session::RedisClient;
use rocket_cors::Cors;
use server::Server;

fn configure_cors() -> Cors {
    Cors {
        ..Default::default()
    }
}

fn initialize(server: &Server) {
    let service = server.initialize_service();
    service.initialize();
}

fn main() {
    dotenv::from_filename("./Config.env").ok();
    let server = server::build_server();
    let cors = configure_cors();
    initialize(&server);
    rocket::ignite()
        .manage(RedisClient::init_pool())
        .manage(server)
        .manage(cors)
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount(
            "admins",
            routes![
                rest::admin::login,
                rest::admin::logout,
                rest::admin::get_admins,
                rest::admin::get_admin,
                rest::admin::register_admin,
                rest::admin::update_admin,
                rest::admin::delete_admin
            ],
        )
        .mount(
            "clients",
            routes![
                rest::client::login,
                rest::client::logout,
                rest::client::get_clients,
                rest::client::get_client,
                rest::client::get_detailed_client,
                rest::client::register_client,
                rest::client::update_client,
                rest::client::delete_client
            ],
        )
        .mount(
            "end_users",
            routes![
                rest::end_user::login,
                rest::end_user::logout,
                rest::end_user::get_end_users,
                rest::end_user::get_end_user,
                rest::end_user::get_detailed_end_user,
                rest::end_user::register_end_user,
                rest::end_user::update_end_user,
                rest::end_user::delete_end_user
            ],
        )
        .mount(
            "resources",
            routes![
                rest::resource::login,
                rest::resource::logout,
                rest::resource::get_resources,
                rest::resource::get_resource,
                rest::resource::get_detailed_resource,
                rest::resource::register_resource,
                rest::resource::update_resource,
                rest::resource::delete_resource,
            ],
        )
        .mount(
            "oidc",
            routes![
                rest::oidc::authorize,
                rest::oidc::accept_client,
                rest::oidc::get_tokens,
                rest::oidc::introspect,
                rest::oidc::get_userinfo,
                rest::oidc::get_publickey,
            ],
        )
        .mount("health", routes![rest::health::get_health,])
        .launch();
}
