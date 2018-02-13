use rocket::request::LenientForm;
use rocket_contrib::Json;
use rocket_cors::{self, Guard};

use app::oidc::{GetTokensCmd, OidcService, OidcServiceComponent};
use domain::error::domain as ed;
use self::ed::ErrorKind as ek;
use domain::constant;
use domain::model::EndUserClaims;
use domain::service::{AcceptClientCmd, AuthorizeCmd, AuthorizeRet, IntrospectCmd, IntrospectRet,
                      TokensRet, UserinfoCmd};
use infra::rest::common::{AuthorizationHeader, AuthorizationType};
use infra::session::RedisStore;
use server::Server;

#[derive(Deserialize, Debug, FromForm)]
pub struct AuthorizeParams {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
}

#[get("/authorize?<authorize_params>")]
pub fn authorize(
    authorize_params: AuthorizeParams,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> AuthorizeRet {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let end_user_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD);
            if let Err(e) = end_user_id {
                return AuthorizeRet::error(e, None, None);
            }
            let cmd = AuthorizeCmd {
                end_user_id: end_user_id.unwrap(),
                client_id: authorize_params.client_id.clone(),
                response_type: authorize_params.response_type.clone(),
                redirect_uri: authorize_params.redirect_uri.clone(),
                scope: authorize_params.scope.clone(),
                state: authorize_params.state.clone(),
                nonce: authorize_params.nonce.clone(),
            };
            let service = server.oidc_service();
            return service.authorize(&cmd);
        }
    }
    AuthorizeRet::error(
        ek::RequireLogin("Login required.".to_string()).into(),
        None,
        None,
    )
}

#[derive(Deserialize, Debug)]
pub struct AcceptanceForm {
    pub action: String,
    pub grant_id: String,
}

#[post("/accept", data = "<input>")]
pub fn accept_client(
    input: Json<AcceptanceForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> AuthorizeRet {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let end_user_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD);
            if let Err(e) = end_user_id {
                return AuthorizeRet::error(e, None, None);
            }
            let form = input.into_inner();
            let cmd = AcceptClientCmd {
                end_user_id: end_user_id.unwrap(),
                action: form.action.clone(),
                grant_id: form.grant_id.clone(),
            };
            let service = server.oidc_service();
            return service.accept_client(&cmd);
        }
    }
    AuthorizeRet::error(
        ek::RequireLogin("Login required.".to_string()).into(),
        None,
        None,
    )
}

#[derive(FromForm, Debug)]
pub struct TokensForm {
    pub grant_type: Option<String>,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[post("/tokens", data = "<input>")]
pub fn get_tokens<'r>(
    cors: Guard<'r>,
    input: LenientForm<TokensForm>,
    authorization_header: AuthorizationHeader,
    server: Server,
) -> rocket_cors::Responder<'r, TokensRet> {
    let (client_id, client_secret) = match authorization_header.get_basic_name_and_password() {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let form = input.into_inner();
    let cmd = GetTokensCmd {
        client_id,
        client_secret,
        grant_type: form.grant_type.clone(),
        code: form.code.clone(),
        refresh_token: form.refresh_token.clone(),
        scope: form.scope.clone(),
        username: form.username.clone(),
        password: form.password.clone(),
    };
    let service = server.oidc_service();
    let ret = service.get_tokens(&cmd);
    cors.responder(ret)
}

#[derive(FromForm, Debug)]
pub struct IntrospectForm {
    pub token: String,
    pub token_type_hint: Option<String>,
}

#[post("/introspect", data = "<input>")]
pub fn introspect<'r>(
    cors: Guard<'r>,
    input: LenientForm<IntrospectForm>,
    authorization_header: AuthorizationHeader,
    server: Server,
) -> rocket_cors::Responder<'r, Result<IntrospectRet, ed::Error>> {
    let (client_id, client_secret) = match authorization_header.get_basic_name_and_password() {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    let form = input.into_inner();
    let cmd = IntrospectCmd {
        client_id,
        client_secret,
        token: form.token.clone(),
        token_type_hint: form.token_type_hint.clone(),
    };
    let service = server.oidc_service();
    let ret = service.introspect_token(&cmd);
    cors.responder(ret)
}

#[get("/userinfo")]
pub fn get_userinfo<'r>(
    cors: Guard<'r>,
    authorization_header: AuthorizationHeader,
    server: Server,
) -> rocket_cors::Responder<'r, Result<EndUserClaims, ed::Error>> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let cmd = UserinfoCmd {
                access_token: token,
            };
            let service = server.oidc_service();
            let ret = service.get_userinfo(&cmd);
            return cors.responder(ret);
        }
    };
    cors.responder(Err(ek::UserinfoError(
        "Access token is required.".to_string(),
    ).into()))
}
