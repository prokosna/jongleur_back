use rocket::request::LenientForm;
use rocket_cors::{Guard, Responder};

use application::oidc::{IntrospectParameters, IntrospectService};
use infra::db::{MongoClient, MongoRepository};
use infra::http::{AuthorizationHeader, DomainResponder};

#[post("/introspect", data = "<input>")]
pub fn post_introspect<'r>(
    cors: Guard<'r>,
    auth_header: AuthorizationHeader,
    input: LenientForm<IntrospectParameters>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let introspect_parameters = input.into_inner();
    let repository = MongoRepository::new(&mongo_client);
    let introspect_service = IntrospectService::<
        MongoRepository,
        MongoRepository,
        MongoRepository,
        MongoRepository,
    >::new();

    let (client_id, secret) = match auth_header.get_basic_name_and_password() {
        Some((id, pass)) => (Some(id), Some(pass)),
        None => (None, None),
    };

    let ret = introspect_service.introspect_token(
        client_id,
        secret,
        &introspect_parameters,
        &repository,
        &repository,
        &repository,
        &repository,
    );

    match ret {
        Ok(resp) => cors.responder(DomainResponder::introspect(resp)),
        Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
    }
}
