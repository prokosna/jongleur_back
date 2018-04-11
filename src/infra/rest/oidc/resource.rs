use self::ed::ErrorKind as ek;
use actix_web::*;
use app::oidc::{GetTokensCmd, OidcService, OidcServiceComponent};
use constant;
use domain::error::domain as ed;
use domain::model::EndUserClaims;
use domain::service::{AcceptClientCmd, AuthorizeCmd, AuthorizeRet, IntrospectCmd, IntrospectRet,
                      TokensRet, UserinfoCmd};
use infra::rest::middleware::AuthorizationType;
use server::ApplicationState;

#[derive(Deserialize, Debug)]
pub struct AuthorizeParams {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
}

pub fn authorize(
    req: HttpRequest<ApplicationState>,
    query: Query<AuthorizeParams>,
) -> AuthorizeRet {
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store();
    let redis_store = match redis_store {
        Ok(s) => s,
        Err(_) => {
            return AuthorizeRet::error(
                ek::TemporarilyUnavailable("Redis cluster is not ready".to_string()).into(),
                None,
                None,
            );
        }
    };
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let end_user_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD);
        if let Err(e) = end_user_id {
            return AuthorizeRet::error(e, None, None);
        }
        let cmd = AuthorizeCmd {
            end_user_id: end_user_id.unwrap(),
            client_id: query.client_id.clone(),
            response_type: query.response_type.clone(),
            redirect_uri: query.redirect_uri.clone(),
            scope: query.scope.clone(),
            state: query.state.clone(),
            nonce: query.nonce.clone(),
        };
        let service = server.oidc_service();
        return service.authorize(&cmd);
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

pub fn accept_client(
    req: HttpRequest<ApplicationState>,
    form: Json<AcceptanceForm>,
) -> AuthorizeRet {
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store();
    let redis_store = match redis_store {
        Ok(s) => s,
        Err(_) => {
            return AuthorizeRet::error(
                ek::TemporarilyUnavailable("Redis cluster is not ready".to_string()).into(),
                None,
                None,
            );
        }
    };
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let end_user_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD);
        if let Err(e) = end_user_id {
            return AuthorizeRet::error(e, None, None);
        }
        let cmd = AcceptClientCmd {
            end_user_id: end_user_id.unwrap(),
            action: form.action.clone(),
            grant_id: form.grant_id.clone(),
        };
        let service = server.oidc_service();
        return service.accept_client(&cmd);
    }
    AuthorizeRet::error(
        ek::RequireLogin("Login required.".to_string()).into(),
        None,
        None,
    )
}

#[derive(Deserialize, Debug)]
pub struct TokensForm {
    pub grant_type: Option<String>,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub fn get_tokens(req: HttpRequest<ApplicationState>, form: Form<TokensForm>) -> TokensRet {
    let server = &req.state().server;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    let (client_id, client_secret) = if let Some(AuthorizationType::Basic { name, password }) = auth
    {
        (Some(name), Some(password))
    } else {
        (None, None)
    };
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
    service.get_tokens(&cmd)
}

#[derive(Deserialize, Debug)]
pub struct IntrospectForm {
    pub token: String,
    pub token_type_hint: Option<String>,
}

pub fn introspect(
    req: HttpRequest<ApplicationState>,
    form: Form<IntrospectForm>,
) -> Result<IntrospectRet, ed::Error> {
    let server = &req.state().server;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    let (client_id, client_secret) = if let Some(AuthorizationType::Basic { name, password }) = auth
    {
        (Some(name), Some(password))
    } else {
        (None, None)
    };
    let cmd = IntrospectCmd {
        client_id,
        client_secret,
        token: form.token.clone(),
        token_type_hint: form.token_type_hint.clone(),
    };
    let service = server.oidc_service();
    service.introspect_token(&cmd)
}

pub fn get_userinfo(req: HttpRequest<ApplicationState>) -> Result<EndUserClaims, ed::Error> {
    let server = &req.state().server;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let cmd = UserinfoCmd {
            access_token: token,
        };
        let service = server.oidc_service();
        return service.get_userinfo(&cmd);
    };
    Err(ek::UserinfoError("Access token is required.".to_string()).into())
}

pub fn get_publickey(req: HttpRequest<ApplicationState>) -> String {
    let server = &req.state().server;
    let service = server.oidc_service();
    service.get_publickey()
}
