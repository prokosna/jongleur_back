use domain::error::domain as ed;
use self::ed::ErrorKind as ek;
use domain::repository::{AccessTokenRepository, AccessTokenRepositoryComponent, ClientRepository,
                         ClientRepositoryComponent, EndUserRepository, EndUserRepositoryComponent,
                         IdTokenRepository, IdTokenRepositoryComponent, RefreshTokenRepository,
                         RefreshTokenRepositoryComponent};
use domain::service::{KeyService, KeyServiceComponent, TokensRet};

pub struct RefreshTokenCmd {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

pub trait RefreshTokenService
    : AccessTokenRepositoryComponent
    + ClientRepositoryComponent
    + EndUserRepositoryComponent
    + IdTokenRepositoryComponent
    + RefreshTokenRepositoryComponent
    + KeyServiceComponent {
    fn refresh_token(&self, cmd: &RefreshTokenCmd) -> TokensRet {
        let access_token_repo = self.access_token_repository();
        let client_repo = self.client_repository();
        let end_user_repo = self.end_user_repository();
        let id_token_repo = self.id_token_repository();
        let refresh_token_repo = self.refresh_token_repository();
        let key_service = self.key_service();

        let ret = process_refresh_token(
            cmd,
            access_token_repo,
            client_repo,
            end_user_repo,
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

pub trait RefreshTokenServiceComponent {
    type RefreshTokenService: RefreshTokenService;
    fn refresh_token_service(&self) -> &Self::RefreshTokenService;
}

impl<
    T: AccessTokenRepositoryComponent
        + ClientRepositoryComponent
        + EndUserRepositoryComponent
        + IdTokenRepositoryComponent
        + RefreshTokenRepositoryComponent
        + KeyServiceComponent,
> RefreshTokenService for T
{
}

// Private functions
fn process_refresh_token(
    cmd: &RefreshTokenCmd,
    access_token_repo: &AccessTokenRepository,
    client_repo: &ClientRepository,
    end_user_repo: &EndUserRepository,
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

    // Refresh token
    let token = cmd.refresh_token
        .as_ref()
        .ok_or::<ed::Error>(ek::InvalidRequest("No refresh_token".to_string()).into())?;
    let refresh_token = refresh_token_repo
        .find_by_token(&token)
        .and_then(|v| v.ok_or(ek::InvalidRequest("Refresh token not found.".to_string()).into()))?;

    if !refresh_token.is_valid() {
        return Err(ek::InvalidRequest("Invalid refresh_token.".to_string()).into());
    }

    // Refresh access token
    let access_token = access_token_repo
        .find_by_id(&refresh_token.access_token_id)
        .and_then(|v| v.ok_or(ek::InvalidRequest("Access token not found.".to_string()).into()))?;
    let access_token = access_token.update();
    access_token_repo.update(&access_token)?;

    // Result
    let mut builder = TokensRet::builder(&access_token);

    // Refresh id token if:
    // 1. Scope has "openid"
    // 2. RefreshToken has id_token_id
    if cmd.scope
        .as_ref()
        .unwrap_or(&"".to_string())
        .split(" ")
        .any(|v| v == "openid") && refresh_token.id_token_id.is_some()
    {
        // Id token
        let id_token = id_token_repo
            .find_by_id(&refresh_token.id_token_id.as_ref().unwrap())
            .and_then(|v| v.ok_or(ek::InvalidRequest("Id token not found.".to_string()).into()))?;

        // EndUser
        let end_user = end_user_repo
            .find_by_id(&id_token.end_user_id)
            .and_then(|v| {
                v.ok_or(
                    ek::InvalidRequest(format!(
                        "EndUser not found. ID => {}",
                        id_token.end_user_id
                    )).into(),
                )
            })?;

        // Refresh id token
        let id_token = id_token.update(
            &client,
            &end_user,
            key_service.jwt_public_key(),
            key_service.jwt_private_key(),
        )?;
        id_token_repo.update(&id_token)?;

        builder = builder.id_token(&Some(id_token));
    }

    builder = builder.refresh_token(&Some(refresh_token));
    Ok(builder.build())
}
