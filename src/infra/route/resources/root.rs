use rocket::http::Cookies;
use rocket_contrib::Json;
use rocket_cors::{Guard, Responder};

use application::resources::{ResourceRegisterForm, ResourceService};
use domain::consts;
use domain::error::domain as ed;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::DomainResponder;

#[get("/")]
pub fn get_resources<'r>(
    cors: Guard<'r>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let repository = MongoRepository::new(&mongo_client);
    let resource_service = ResourceService::<RedisStore, MongoRepository>::new();
    let ret = resource_service.get_resources(&repository);
    match ret {
        Ok(resources) => cors.responder(DomainResponder::resources(resources)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[get("/<id>")]
pub fn get_resource<'r>(
    cors: Guard<'r>,
    id: String,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let repository = MongoRepository::new(&mongo_client);
    let resource_service = ResourceService::<RedisStore, MongoRepository>::new();
    let ret = resource_service.get_resource(&id, &repository);
    match ret {
        Ok(resource) => cors.responder(DomainResponder::resource(resource)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[get("/private/<id>")]
pub fn get_private_resource(
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
        let resource_service = ResourceService::<RedisStore, MongoRepository>::new();
        let ret = resource_service.get_private_resource(&id, &s, &store, &repository);
        match ret {
            Ok(resource) => DomainResponder::raw_resource(resource),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}

#[post("/", data = "<input>")]
pub fn post_resource<'r>(
    cors: Guard<'r>,
    input: Json<ResourceRegisterForm>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let register_form = input.into_inner();
    let repository = MongoRepository::new(&mongo_client);
    let resource_service = ResourceService::<RedisStore, MongoRepository>::new();
    let ret = resource_service.register_resource(register_form, &repository);
    match ret {
        Ok(_) => cors.responder(DomainResponder::ok(
            Some("The resource was registered.".to_string()),
        )),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[put("/private/<id>", data = "<input>")]
pub fn put_private_resource(
    id: String,
    input: Json<ResourceRegisterForm>,
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
        let resource_service = ResourceService::<RedisStore, MongoRepository>::new();
        let ret =
            resource_service.update_private_resource(&id, &s, register_form, &store, &repository);
        match ret {
            Ok(_) => DomainResponder::ok(Some("The resource was updated.".to_string())),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}

#[delete("/private/<id>")]
pub fn delete_private_resource(
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
        let resource_service = ResourceService::<RedisStore, MongoRepository>::new();
        let ret = resource_service.delete_private_resource(&id, &s, &store, &repository);
        match ret {
            Ok(_) => DomainResponder::ok(Some("The resource was deleted.".to_string())),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}
