use rocket::http::Cookies;
use rocket_contrib::Json;
use rocket_cors::{Guard, Responder};

use application::clients::{ClientRegisterForm, ClientService};
use domain::consts;
use domain::error::domain as ed;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::DomainResponder;

#[get("/")]
pub fn get_clients<'r>(
    cors: Guard<'r>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let repository = MongoRepository::new(&mongo_client);
    let client_service = ClientService::<RedisStore, MongoRepository>::new();
    let ret = client_service.get_clients(&repository);
    match ret {
        Ok(clients) => cors.responder(DomainResponder::clients(clients)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[get("/<id>")]
pub fn get_client<'r>(
    cors: Guard<'r>,
    id: String,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let repository = MongoRepository::new(&mongo_client);
    let client_service = ClientService::<RedisStore, MongoRepository>::new();
    let ret = client_service.get_client(&id, &repository);
    match ret {
        Ok(client) => cors.responder(DomainResponder::client(client)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[get("/private/<id>")]
pub fn get_private_client(
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
        let client_service = ClientService::<RedisStore, MongoRepository>::new();
        let ret = client_service.get_private_client(&id, &s, &store, &repository);
        match ret {
            Ok(client) => DomainResponder::raw_client(client),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}

#[post("/", data = "<input>")]
pub fn post_client<'r>(
    cors: Guard<'r>,
    input: Json<ClientRegisterForm>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let register_form = input.into_inner();
    let repository = MongoRepository::new(&mongo_client);
    let client_service = ClientService::<RedisStore, MongoRepository>::new();
    let ret = client_service.register_client(register_form, &repository);
    match ret {
        Ok(_) => cors.responder(DomainResponder::ok(
            Some("The client was registered.".to_string()),
        )),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}

#[put("/private/<id>", data = "<input>")]
pub fn put_private_client(
    id: String,
    input: Json<ClientRegisterForm>,
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
        let client_service = ClientService::<RedisStore, MongoRepository>::new();
        let ret = client_service.update_private_client(&id, &s, register_form, &store, &repository);
        match ret {
            Ok(_) => DomainResponder::ok(Some("The client was updated.".to_string())),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}

#[delete("/private/<id>")]
pub fn delete_private_client(
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
        let client_service = ClientService::<RedisStore, MongoRepository>::new();
        let ret = client_service.delete_private_client(&id, &s, &store, &repository);
        match ret {
            Ok(_) => DomainResponder::ok(Some("The client was deleted.".to_string())),
            Err(e) => DomainResponder::from_domain_error(e),
        }
    } else {
        DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into())
    }
}
