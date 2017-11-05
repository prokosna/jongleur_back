use rocket::request::LenientForm;
use rocket_cors::{Guard, Responder};

use application::oidc::{TokensParameters, TokensService};
use infra::db::{MongoClient, MongoRepository};
use infra::http::{AuthorizationHeader, DomainResponder};

#[post("/tokens", data = "<input>")]
pub fn post_tokens<'r>(
    cors: Guard<'r>,
    auth_header: AuthorizationHeader,
    input: LenientForm<TokensParameters>,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    let tokens_parameters = input.into_inner();

    let repository = MongoRepository::new(&mongo_client);
    let tokens_service = TokensService::<
        MongoRepository,
        MongoRepository,
        MongoRepository,
        MongoRepository,
        MongoRepository,
        MongoRepository,
    >::new();

    let (client_id, secret) = match auth_header.get_basic_name_and_password() {
        Some((id, pass)) => (Some(id), Some(pass)),
        None => (None, None),
    };

    let ret = tokens_service.get_tokens(
        client_id,
        secret,
        &tokens_parameters,
        &repository,
        &repository,
        &repository,
        &repository,
        &repository,
        &repository,
    );

    cors.responder(DomainResponder::from_tokens_domain(ret))
}
