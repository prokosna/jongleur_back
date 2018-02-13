use domain::error::domain as ed;
use self::ed::ErrorKind as ek;
use domain::model::{EndUserClaims, GrantType};
use domain::service::{AcceptClientCmd, AcceptGrantCmd, AuthorizeCmd, AuthorizeRet,
                      AuthorizeService, AuthorizeServiceComponent, ClientCredentialsCmd,
                      ClientCredentialsService, ClientCredentialsServiceComponent, IntrospectCmd,
                      IntrospectRet, IntrospectService, IntrospectServiceComponent, KeyService,
                      KeyServiceComponent, RefreshTokenCmd, RefreshTokenService,
                      RefreshTokenServiceComponent, ResourceOwnerPasswordCredentialsCmd,
                      ResourceOwnerPasswordCredentialsService,
                      ResourceOwnerPasswordCredentialsServiceComponent, TokensRet, UserinfoCmd,
                      UserinfoService, UserinfoServiceComponent};

pub struct GetTokensCmd {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub grant_type: Option<String>,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub trait OidcService
    : AuthorizeServiceComponent
    + ClientCredentialsServiceComponent
    + IntrospectServiceComponent
    + RefreshTokenServiceComponent
    + ResourceOwnerPasswordCredentialsServiceComponent
    + UserinfoServiceComponent
    + KeyServiceComponent {
    fn authorize(&self, cmd: &AuthorizeCmd) -> AuthorizeRet {
        let service = self.authorize_service();
        service.authorize(cmd)
    }

    fn accept_client(&self, cmd: &AcceptClientCmd) -> AuthorizeRet {
        let service = self.authorize_service();
        service.accept_client(cmd)
    }

    fn get_tokens(&self, cmd: &GetTokensCmd) -> TokensRet {
        if cmd.grant_type.is_none() {
            return TokensRet::error(
                ek::InvalidRequest("grant_type is required".to_string()).into(),
            );
        }

        let grant_type = GrantType::new(&cmd.grant_type.as_ref().unwrap());

        match grant_type {
            GrantType::AuthorizationCode => {
                let service = self.authorize_service();
                let cmd = AcceptGrantCmd {
                    client_id: cmd.client_id.clone(),
                    client_secret: cmd.client_secret.clone(),
                    code: cmd.code.clone(),
                };
                service.accept_grant(&cmd)
            }
            GrantType::RefreshToken => {
                let service = self.refresh_token_service();
                let cmd = RefreshTokenCmd {
                    client_id: cmd.client_id.clone(),
                    client_secret: cmd.client_secret.clone(),
                    refresh_token: cmd.refresh_token.clone(),
                    scope: cmd.scope.clone(),
                };
                service.refresh_token(&cmd)
            }
            GrantType::ClientCredentials => {
                let service = self.client_credentials_service();
                let cmd = ClientCredentialsCmd {
                    client_id: cmd.client_id.clone(),
                    client_secret: cmd.client_secret.clone(),
                    scope: cmd.scope.clone(),
                };
                service.execute_client_credentials(&cmd)
            }
            GrantType::Password => {
                let service = self.resource_owner_password_credentials_service();
                let cmd = ResourceOwnerPasswordCredentialsCmd {
                    client_id: cmd.client_id.clone(),
                    client_secret: cmd.client_secret.clone(),
                    username: cmd.username.clone(),
                    password: cmd.password.clone(),
                    scope: cmd.scope.clone(),
                };
                service.execute_resource_owner_password_credentials(&cmd)
            }
            GrantType::Undefined(ref raw) => TokensRet::error(
                ek::UnsupportedGrantType(format!("Unsupported grant_type. {}", raw)).into(),
            ),
        }
    }

    fn introspect_token(&self, cmd: &IntrospectCmd) -> Result<IntrospectRet, ed::Error> {
        let service = self.introspect_service();
        service.introspect(cmd)
    }

    fn get_userinfo(&self, cmd: &UserinfoCmd) -> Result<EndUserClaims, ed::Error> {
        let service = self.userinfo_service();
        service.get_userinfo(cmd)
    }

    fn get_publickey(&self) -> String {
        let service = self.key_service();
        service.jwt_public_key_pem().clone()
    }
}

pub trait OidcServiceComponent {
    type OidcService: OidcService;
    fn oidc_service(&self) -> &Self::OidcService;
}

// Implement
impl<
    T: AuthorizeServiceComponent
        + ClientCredentialsServiceComponent
        + IntrospectServiceComponent
        + RefreshTokenServiceComponent
        + ResourceOwnerPasswordCredentialsServiceComponent
        + UserinfoServiceComponent
        + KeyServiceComponent,
> OidcService for T
{
}
