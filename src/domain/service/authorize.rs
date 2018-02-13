use config::AppConfig;
use constant;
use domain::error::domain as ed;
use self::ed::ErrorKind as ek;
use domain::model::{AccessToken, Client, EndUser, FlowType, Grant, GrantStatus, IdTokenClaims,
                    RefreshToken, Resource, ResponseType};
use domain::repository::{AccessTokenRepository, AccessTokenRepositoryComponent, ClientRepository,
                         ClientRepositoryComponent, EndUserRepository, EndUserRepositoryComponent,
                         GrantRepository, GrantRepositoryComponent, IdTokenRepository,
                         IdTokenRepositoryComponent, RefreshTokenRepository,
                         RefreshTokenRepositoryComponent, ResourceRepository,
                         ResourceRepositoryComponent};
use domain::service::{AuthorizeRet, KeyService, KeyServiceComponent, TokensRet};

pub struct AuthorizeCmd {
    pub end_user_id: Option<String>,
    pub client_id: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
}

pub struct AcceptClientCmd {
    pub end_user_id: Option<String>,
    pub action: String,
    pub grant_id: String,
}

pub struct AcceptGrantCmd {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub code: Option<String>,
}

/// `AuthorizeService` provides functions for Authorization Code Flow.
pub trait AuthorizeService
    : AccessTokenRepositoryComponent
    + ClientRepositoryComponent
    + EndUserRepositoryComponent
    + GrantRepositoryComponent
    + IdTokenRepositoryComponent
    + RefreshTokenRepositoryComponent
    + ResourceRepositoryComponent
    + KeyServiceComponent {
    /// Execute Authorization Code flow.
    fn authorize(&self, cmd: &AuthorizeCmd) -> AuthorizeRet {
        let access_token_repo = self.access_token_repository();
        let client_repo = self.client_repository();
        let end_user_repo = self.end_user_repository();
        let grant_repo = self.grant_repository();
        let id_token_repo = self.id_token_repository();
        let resource_repo = self.resource_repository();
        let key_service = self.key_service();

        // Check login
        let end_user = match cmd.end_user_id {
            Some(ref id) => match end_user_repo.find_by_id(id) {
                Ok(ret) => match ret {
                    Some(end_user) => end_user,
                    None => {
                        return AuthorizeRet::error(
                            ek::EntityNotFound(format!("End user not found. ID => {}", id)).into(),
                            None,
                            None,
                        );
                    }
                },
                Err(e) => return AuthorizeRet::error(e, None, None),
            },
            None => {
                return AuthorizeRet::error(
                    ek::RequireLogin("Login required.".to_string()).into(),
                    None,
                    None,
                );
            }
        };

        // Find client
        // We dunno whether the redirect_uri is valid yet.
        let client = match client_repo.find_by_id(&cmd.client_id).and_then(|v| {
            v.ok_or(ek::EntityNotFound(format!("Client not found. ID => {}", cmd.client_id)).into())
        }) {
            Ok(c) => c,
            Err(e) => return AuthorizeRet::error(e, None, cmd.state.clone()),
        };

        // First of all, we should check whether the redirect_uri is valid or not.
        // If the redirect_uri is invalid, we MUST NOT send any data to that uri.
        let redirect_uri = match validate_redirect_uri(&cmd.redirect_uri, &client) {
            Ok(uri) => uri,
            Err(e) => return AuthorizeRet::error(e, None, cmd.state.clone()),
        };

        // Now we know that the redirect_uri is valid.
        // From now, any errors are wrapped into the AuthorizationCodeRet.
        let ret = execute_authorization_code_flow(
            cmd,
            client,
            end_user,
            &redirect_uri,
            access_token_repo,
            grant_repo,
            id_token_repo,
            resource_repo,
            key_service,
        );
        match ret {
            Ok(r) => r,
            Err(e) => AuthorizeRet::error(e, Some(redirect_uri), cmd.state.clone()),
        }
    }

    /// Execute Authorization Code accepting client flow.
    fn accept_client(&self, cmd: &AcceptClientCmd) -> AuthorizeRet {
        let access_token_repo = self.access_token_repository();
        let end_user_repo = self.end_user_repository();
        let grant_repo = self.grant_repository();
        let id_token_repo = self.id_token_repository();
        let key_service = self.key_service();

        let grant = match grant_repo.find_by_id(&cmd.grant_id).and_then(|v| {
            v.ok_or(ek::InvalidRequest(format!("Invalid grant_id. ID => {}", cmd.grant_id)).into())
        }) {
            Ok(g) => g,
            Err(e) => return AuthorizeRet::error(e, None, None),
        };

        let ret = execute_accepting_client(
            cmd,
            &grant,
            access_token_repo,
            end_user_repo,
            grant_repo,
            id_token_repo,
            key_service,
        );
        match ret {
            Ok(r) => r,
            Err(e) => AuthorizeRet::error(e, Some(grant.redirect_uri), grant.state),
        }
    }

    /// Check the grant code and
    fn accept_grant(&self, cmd: &AcceptGrantCmd) -> TokensRet {
        let access_token_repo = self.access_token_repository();
        let client_repo = self.client_repository();
        let end_user_repo = self.end_user_repository();
        let grant_repo = self.grant_repository();
        let id_token_repo = self.id_token_repository();
        let refresh_token_repo = self.refresh_token_repository();
        let key_service = self.key_service();

        let ret = execute_accepting_grant(
            cmd,
            access_token_repo,
            client_repo,
            end_user_repo,
            grant_repo,
            id_token_repo,
            refresh_token_repo,
            key_service,
        );
        match ret {
            Ok(r) => r,
            Err(e) => TokensRet::error(e),
        }
    }
}

pub trait AuthorizeServiceComponent {
    type AuthorizeService: AuthorizeService;
    fn authorize_service(&self) -> &Self::AuthorizeService;
}

// Implement
impl<
    T: AccessTokenRepositoryComponent
        + ClientRepositoryComponent
        + EndUserRepositoryComponent
        + GrantRepositoryComponent
        + IdTokenRepositoryComponent
        + RefreshTokenRepositoryComponent
        + ResourceRepositoryComponent
        + KeyServiceComponent,
> AuthorizeService for T
{
}

// Private functions
/// Execute Authorization Code flow.
fn execute_authorization_code_flow(
    cmd: &AuthorizeCmd,
    client: Client,
    end_user: EndUser,
    redirect_uri: &String,
    access_token_repo: &AccessTokenRepository,
    grant_repo: &GrantRepository,
    id_token_repo: &IdTokenRepository,
    resource_repo: &ResourceRepository,
    key_service: &KeyService,
) -> Result<AuthorizeRet, ed::Error> {
    let resource = resource_repo
        .find_by_id(&client.resource_id)
        .and_then(|v| v.ok_or(ek::EntityNotFound(format!("ID => {}", client.resource_id)).into()))?;

    // Validation
    let response_type = validate_response_type(&cmd.response_type)?;
    let scope = cmd.scope
        .as_ref()
        .map_or(Vec::new(), |v| validate_scope_str(v, &resource));

    // Generate Grant
    let grant = Grant::builder(
        &end_user.id,
        &cmd.client_id,
        &client.resource_id,
        &response_type,
        redirect_uri,
    ).state(&cmd.state)
        .nonce(&cmd.nonce)
        .scope(&scope)
        .build();
    grant_repo.add(&grant)?;

    // Require new acceptance?
    if end_user.require_acceptance(&scope, &cmd.client_id) {
        // Return and request new acceptance
        let ret = AuthorizeRet::require_acceptance(grant.id, resource.convert_scope(&scope));
        Ok(ret)
    } else {
        // Already accepted, so go on
        generate_code_or_tokens(
            &end_user,
            &grant.id,
            access_token_repo,
            grant_repo,
            id_token_repo,
            key_service,
        )
    }
}

/// Check parameters and add the new accepted client to the end user.
fn execute_accepting_client(
    cmd: &AcceptClientCmd,
    grant: &Grant,
    access_token_repo: &AccessTokenRepository,
    end_user_repo: &EndUserRepository,
    grant_repo: &GrantRepository,
    id_token_repo: &IdTokenRepository,
    key_service: &KeyService,
) -> Result<AuthorizeRet, ed::Error> {
    // Check login
    let mut end_user = match cmd.end_user_id {
        Some(ref id) => end_user_repo.find_by_id(id).and_then(|v| {
            v.ok_or(ek::EntityNotFound(format!("End user not found. ID => {}", id)).into())
        })?,
        None => {
            return Err(ek::RequireLogin("Login required.".to_string()).into());
        }
    };

    if cmd.action != constant::ACTION_ACCEPT {
        return Err(ek::AccessDenied("The user rejected the request.".to_string()).into());
    }

    if end_user.id != grant.end_user_id {
        return Err(ek::AccessDenied(format!(
            "The granted user does not match. EndUser => {}",
            end_user.id
        )).into());
    }

    // Update end_user
    end_user.add_accepted_client(&grant.client_id, &grant.scope);
    end_user_repo.update(&end_user)?;

    // Generate tokens
    generate_code_or_tokens(
        &end_user,
        &cmd.grant_id,
        access_token_repo,
        grant_repo,
        id_token_repo,
        key_service,
    )
}

/// Update grant status, then generate tokens.
fn generate_code_or_tokens(
    end_user: &EndUser,
    grant_id: &String,
    access_token_repo: &AccessTokenRepository,
    grant_repo: &GrantRepository,
    id_token_repo: &IdTokenRepository,
    key_service: &KeyService,
) -> Result<AuthorizeRet, ed::Error> {
    let grant = grant_repo
        .find_by_id_and_change_status(grant_id, &GrantStatus::Activated)
        .and_then(|v| {
            v.ok_or(ek::InvalidRequest(format!("Grant not found. ID => {}", grant_id)).into())
        })?;

    // ID of the grant MUST be the same with ID of the end user.
    if grant.end_user_id != end_user.id {
        return Err(ek::AccessDenied(format!(
            "The granted user does not match. EndUser => {}",
            end_user.id
        )).into());
    }

    // If the status of the grant was not "Created", it has been already used.
    if grant.status != GrantStatus::Created {
        return Err(ek::InvalidRequest(format!(
            "The grant has been already used. ID => {}",
            grant.id
        )).into());
    }

    // Grant is valid or not
    if !grant.is_valid() {
        return Err(ek::InvalidGrant("Grant has been already expired.".to_string()).into());
    }

    // Grant code
    let code = if grant.response_type.has_code() {
        Some(grant.code.to_string())
    } else {
        None
    };

    // Access token
    let access_token = if grant.response_type.has_token() {
        let access_token = AccessToken::builder(&grant.client_id, &grant.resource_id)
            .end_user_id(&Some(grant.end_user_id.clone()))
            .state(&grant.state)
            .scope(&grant.scope)
            .nonce(&grant.nonce)
            .build();
        access_token_repo.add(&access_token)?;
        Some(access_token)
    } else {
        None
    };

    // Id token
    let id_token = if grant.response_type.has_id_token() {
        let id_token =
            IdTokenClaims::from_end_user(&AppConfig::issuer(), end_user, &grant.client_id)
                .nonce(&grant.nonce)
                .publish(key_service.jwt_private_key())?;
        id_token_repo.add(&id_token)?;
        Some(id_token)
    } else {
        None
    };

    // Result
    AuthorizeRet::builder(grant.redirect_uri.clone(), grant.state.clone())
        .code(code)
        .access_token(access_token)
        .id_token(id_token)
        .build()
        .ok_or(ek::InvalidRequest("Correct response type not found.".to_string()).into())
}

/// Check the grant code and return tokens
fn execute_accepting_grant(
    cmd: &AcceptGrantCmd,
    access_token_repo: &AccessTokenRepository,
    client_repo: &ClientRepository,
    end_user_repo: &EndUserRepository,
    grant_repo: &GrantRepository,
    id_token_repo: &IdTokenRepository,
    refresh_token_repo: &RefreshTokenRepository,
    key_service: &KeyService,
) -> Result<TokensRet, ed::Error> {
    // Authorize the client
    let (client_id, client_secret) = cmd.client_id
        .as_ref()
        .and_then(|id| cmd.client_secret.as_ref().map(|secret| (id, secret)))
        .ok_or(ek::UnauthorizedClient("Invalid credentials.".to_string()))?;

    let client = client_repo.find_by_id(&client_id).and_then(|c| {
        c.ok_or(ek::UnauthorizedClient(format!("Client not found. ID => {}", client_id)).into())
    })?;

    if !client.is_authenticated_by_secret(&client_secret) {
        return Err(
            ek::UnauthorizedClient(format!("Client not authorized. ID => {}", client_id)).into(),
        );
    }

    // Fetch grant
    let grant = match cmd.code {
        Some(ref c) => grant_repo
            .find_by_code_and_change_status(c, &GrantStatus::Expired)
            .and_then(|v| {
                v.ok_or(ek::InvalidRequest(format!("Grant not found. Code => {}", c)).into())
            })?,
        None => return Err(ek::InvalidGrant("Invalid grant code.".to_string()).into()),
    };

    // EndUser
    let end_user = end_user_repo.find_by_id(&grant.end_user_id).and_then(|v| {
        v.ok_or(ek::InvalidGrant(format!("EndUser not found. ID => {}", grant.end_user_id)).into())
    })?;

    // ID of the grant MUST be the same with ID of the end user.
    if grant.end_user_id != end_user.id {
        return Err(ek::AccessDenied(format!(
            "The granted user does not match. EndUser => {}",
            end_user.id
        )).into());
    }

    // If the status of the grant was not "Activated", it has been already used.
    if grant.status != GrantStatus::Activated {
        return Err(ek::InvalidRequest(format!(
            "The grant has been already used. ID => {}",
            grant.id
        )).into());
    }

    // Grant is valid or not
    if !grant.is_valid() {
        return Err(ek::InvalidGrant("Grant has been already expired.".to_string()).into());
    }

    // Access token is necessary
    // TODO: Should we re-check the scope here?
    let access_token = AccessToken::builder(&grant.client_id, &grant.resource_id)
        .end_user_id(&Some(grant.end_user_id.clone()))
        .state(&grant.state)
        .scope(&grant.scope)
        .nonce(&grant.nonce)
        .build();
    access_token_repo.add(&access_token)?;

    // Id token
    let id_token = if grant.response_type.has_id_token() {
        let id_token =
            IdTokenClaims::from_end_user(&AppConfig::issuer(), &end_user, &grant.client_id)
                .nonce(&grant.nonce)
                .publish(key_service.jwt_private_key())?;
        id_token_repo.add(&id_token)?;
        Some(id_token)
    } else {
        None
    };

    // Refresh token
    let refresh_token =
        RefreshToken::new(&access_token.id, &id_token.as_ref().map(|v| v.id.clone()));
    refresh_token_repo.add(&refresh_token)?;

    Ok(TokensRet::builder(&access_token)
        .id_token(&id_token)
        .refresh_token(&Some(refresh_token))
        .build())
}

/// Validate the string that represents response_type, then return `ResponseType` if the string is valid.
fn validate_response_type(response_type: &String) -> Result<ResponseType, ed::Error> {
    let response_type = ResponseType::from_str(response_type)?;
    match response_type.flow_type() {
        FlowType::Undefined => Err(ek::InvalidRequest("Invalid response_type.".to_string()).into()),
        _ => Ok(response_type),
    }
}

/// Validate the string that represents redirect_uri with the Client, then return valid redirect_uri.
fn validate_redirect_uri(redirect_uri: &String, client: &Client) -> Result<String, ed::Error> {
    if !client.validate_redirect_uri(redirect_uri) {
        Err(ek::InvalidRequest("Invalid redirect_uri.".to_string()).into())
    } else {
        Ok(redirect_uri.to_string())
    }
}

/// Validate the scope and return valid scope.
fn validate_scope_str(scope: &String, resource: &Resource) -> Vec<String> {
    let scope = scope.split(" ").map(|s| s.to_string()).collect();
    resource.filter_scope(&scope)
}
