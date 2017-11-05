use rocket_cors::{Guard, Responder};

use domain::error::domain as ed;
use application::oidc::UserinfoService;
use infra::db::{MongoClient, MongoRepository};
use infra::http::{AuthorizationHeader, AuthorizationType, DomainResponder};

#[get("/userinfo")]
pub fn get_userinfo<'r>(
    cors: Guard<'r>,
    auth_header: AuthorizationHeader,
    mongo_client: MongoClient,
) -> Responder<'r, DomainResponder> {
    match auth_header.auth_type {
        AuthorizationType::Bearer if auth_header.token.is_some() => {
            let access_token = auth_header.token.unwrap();
            let repository = MongoRepository::new(&mongo_client);
            let userinfo_service = UserinfoService::<MongoRepository, MongoRepository>::new();
            let ret = userinfo_service.get_userinfo(&access_token, &repository, &repository);
            match ret {
                Ok(claims) => cors.responder(DomainResponder::userinfo(claims)),
                Err(e) => cors.responder(DomainResponder::from_domain_error(e)),
            }
        }
        _ => cors.responder(DomainResponder::from_domain_error(
            ed::ErrorKind::UserinfoError("Access Token is required.".to_string()).into(),
        )),
    }
}
