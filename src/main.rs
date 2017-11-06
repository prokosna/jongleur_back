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
extern crate env_logger;
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
extern crate uuid;

mod application;
mod domain;
mod infra;
mod util;

use infra::http::get_cors_options;
use infra::route;
use infra::db::MongoClient;
use infra::session::RedisClient;

fn main() {
    dotenv::from_filename("./Config.env").ok();
    let cors_options = get_cors_options();
    rocket::ignite()
        .manage(MongoClient::init_pool())
        .manage(RedisClient::init_pool())
        .manage(cors_options)
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount(
            "clients",
            routes![
                route::clients::get_clients,
                route::clients::get_client,
                route::clients::get_private_client,
                route::clients::post_client,
                route::clients::put_private_client,
                route::clients::delete_private_client,
                route::clients::post_login,
                route::clients::get_logout
            ],
        )
        .mount(
            "resources",
            routes![
                route::resources::get_resources,
                route::resources::get_resource,
                route::resources::get_private_resource,
                route::resources::post_resource,
                route::resources::put_private_resource,
                route::resources::delete_private_resource,
                route::resources::post_login,
                route::resources::get_logout,
            ],
        )
        .mount(
            "end_users",
            routes![
                route::end_users::get_end_users,
                route::end_users::get_end_user,
                route::end_users::get_private_end_user,
                route::end_users::post_end_user,
                route::end_users::put_private_end_user,
                route::end_users::delete_private_end_user,
                route::end_users::post_login,
                route::end_users::get_logout
            ],
        )
        .mount(
            "oidc",
            routes![
                route::oidc::get_authorize,
                route::oidc::post_accept,
                route::oidc::post_tokens,
                route::oidc::get_key_pem,
                route::oidc::get_userinfo,
                route::oidc::post_introspect,
            ],
        )
        .launch();
}
