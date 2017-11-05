use rocket::http::Cookies;
use rocket_contrib::Json;

use application::oidc::AuthorizeService;
use domain::consts;
use domain::error::domain as ed;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::{DomainParameters, DomainResponder};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AcceptanceForm {
    pub action: String,
    pub grant_id: String,
}

#[post("/accept", data = "<input>")]
pub fn post_accept(
    input: Json<AcceptanceForm>,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    cookies: Cookies,
) -> DomainResponder {
    let acceptance_form = input.into_inner();
    let sid = cookies
        .get(consts::COOKIE_KEY)
        .map(|ref c| c.value().to_string());

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

    let ret =
        authorize_service.pre_process_accept(&acceptance_form.grant_id, &repository, &repository);
    let (redirect_uri, _client, grant) = match ret {
        Ok(x) => x,
        Err(e) => return DomainResponder::from_domain_error(e),
    };

    let domain_error_parameters =
        DomainParameters::new(Some(redirect_uri.clone()), grant.state.clone());

    if let Some(s) = sid {
        let ret = authorize_service.process_accept(
            &s,
            &acceptance_form.action,
            &redirect_uri,
            grant,
            &store,
            &repository,
            &repository,
            &repository,
        );
        DomainResponder::from_auth_domain(ret, Some(domain_error_parameters))
    } else {
        if acceptance_form.action != consts::ACTION_ACCEPT {
            DomainResponder::from_auth_domain(
                Err(
                    ed::ErrorKind::AccessDenied("The user rejected the request.".to_string())
                        .into(),
                ),
                Some(domain_error_parameters),
            )
        } else {
            DomainResponder::from_auth_domain(
                Err(ed::ErrorKind::RequireLogin.into()),
                Some(domain_error_parameters),
            )
        }
    }
}
