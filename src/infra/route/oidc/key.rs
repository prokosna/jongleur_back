use rocket_cors::{Guard, Responder};

use infra::http::DomainResponder;
use util::KeyStore;

#[get("/key_pem")]
pub fn get_key_pem<'r>(cors: Guard<'r>) -> Responder<'r, DomainResponder> {
    cors.responder(DomainResponder::key(KeyStore::jwt_public_key_pem().clone()))
}
