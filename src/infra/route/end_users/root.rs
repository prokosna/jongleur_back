use rocket::http::Cookies;
use rocket_contrib::Json;
use rocket_cors::{Guard, Responder};

use application::end_users::{EndUserRegisterForm, EndUserService};
use domain::consts;
use domain::error::domain as ed;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::DomainResponder;

#[get("/")]
pub fn get_end_users<'r>(
    cors: Guard<'r>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let repository = MongoRepository::new(&mongo_client);
    let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
    let ret = end_user_service.get_end_users(&repository);
    match ret {
        Ok(end_users) => cors.responder(DomainResponder::end_users(end_users)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[get("/<id>")]
pub fn get_end_user<'r>(
    cors: Guard<'r>,
    id: String,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let repository = MongoRepository::new(&mongo_client);
    let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
    let ret = end_user_service.get_end_user(&id, &repository);
    match ret {
        Ok(end_user) => cors.responder(DomainResponder::end_user(end_user)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}
#[get("/private/<id>")]
pub fn get_private_end_user(
    id: String,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    cookies: Cookies,
) -> DomainResponder {
    let sid = cookies
        .get(consts::COOKIE_KEY)
        .map(|ref c| c.value().to_string());
    if let Some(s) = sid {
        let store = RedisStore::new(&redis_client);
        let repository = MongoRepository::new(&mongo_client);
        let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
        let ret = end_user_service.get_private_end_user(&id, &s, &store, &repository);
        match ret {
            Ok(end_user) => DomainResponder::raw_end_user(end_user),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}

#[post("/", data = "<input>")]
pub fn post_end_user<'r>(
    cors: Guard<'r>,
    mongo_client: MongoClient,
    input: Json<EndUserRegisterForm>,
) -> Responder<'r, DomainResponder> {
    let register_form = input.into_inner();
    let repository = MongoRepository::new(&mongo_client);
    let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
    let ret = end_user_service.register_end_user(register_form, &repository);
    match ret {
        Ok(_) => cors.responder(DomainResponder::ok(
            Some("The user was registered.".to_string()),
        )),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[put("/private/<id>", data = "<input>")]
pub fn put_private_end_user(
    id: String,
    input: Json<EndUserRegisterForm>,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    cookies: Cookies,
) -> DomainResponder {
    let sid = cookies
        .get(consts::COOKIE_KEY)
        .map(|ref c| c.value().to_string());
    if let Some(s) = sid {
        let register_form = input.into_inner();
        let store = RedisStore::new(&redis_client);
        let repository = MongoRepository::new(&mongo_client);
        let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
        let ret =
            end_user_service.update_private_end_user(&id, &s, register_form, &store, &repository);
        match ret {
            Ok(_) => DomainResponder::ok(Some("The end-user was updated.".to_string())),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}

#[delete("/private/<id>")]
pub fn delete_private_end_user(
    id: String,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    cookies: Cookies,
) -> DomainResponder {
    let sid = cookies
        .get(consts::COOKIE_KEY)
        .map(|ref c| c.value().to_string());
    if let Some(s) = sid {
        let store = RedisStore::new(&redis_client);
        let repository = MongoRepository::new(&mongo_client);
        let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
        let ret = end_user_service.delete_private_end_user(&id, &s, &store, &repository);
        match ret {
            Ok(_) => DomainResponder::ok(Some("The end-user was deleted.".to_string())),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}
