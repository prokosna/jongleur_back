use rocket::http::Cookies;

use application::oidc::{AuthParameters, AuthorizeService};
use domain::consts;
use domain::error::domain as ed;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::{DomainParameters, DomainResponder};

#[get("/authorize?<auth_parameters>")]
pub fn get_authorize(
    auth_parameters: AuthParameters,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    cookies: Cookies,
) -> DomainResponder {
    let sid = cookies
        .get(consts::COOKIE_KEY)
        .map(|ref c| c.value().to_string());

    match sid {
        Some(s) => {
            let store = RedisStore::new(&redis_client);
            let repository = MongoRepository::new(&mongo_client);
            let authorize_service = AuthorizeService::<
                MongoRepository,
                MongoRepository,
                MongoRepository,
                MongoRepository,
                MongoRepository,
                RedisStore,
                MongoRepository,
                MongoRepository,
            >::new();

            let ret = authorize_service.pre_process_auth(&auth_parameters, &repository);
            let (redirect_uri, client) = match ret {
                Ok(x) => x,
                Err(e) => return DomainResponder::from_domain_error(e),
            };

            let domain_error_parameters =
                DomainParameters::new(Some(redirect_uri.clone()), auth_parameters.state.clone());

            let ret = authorize_service.process_auth(
                &s,
                &client,
                &redirect_uri,
                &auth_parameters,
                &store,
                &repository,
                &repository,
                &repository,
                &repository,
            );

            DomainResponder::from_auth_domain(ret, Some(domain_error_parameters))
        }
        None => DomainResponder::from_domain_error(ed::ErrorKind::RequireLogin.into()),
    }
}
