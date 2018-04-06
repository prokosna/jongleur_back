use self::ed::ErrorKind as ek;
use domain::error::domain as ed;
use domain::model::{AccessToken, Resource};
use domain::repository::{AccessTokenRepository, AccessTokenRepositoryComponent, ClientRepository,
                         ClientRepositoryComponent, ResourceRepository,
                         ResourceRepositoryComponent};
use domain::service::TokensRet;

pub struct ClientCredentialsCmd {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scope: Option<String>,
}

pub trait ClientCredentialsService:
    AccessTokenRepositoryComponent + ClientRepositoryComponent + ResourceRepositoryComponent
{
    fn execute_client_credentials(&self, cmd: &ClientCredentialsCmd) -> TokensRet {
        let access_token_repo = self.access_token_repository();
        let client_repo = self.client_repository();
        let resource_repo = self.resource_repository();

        let ret = process_client_credentials(cmd, access_token_repo, client_repo, resource_repo);

        match ret {
            Ok(r) => r,
            Err(e) => TokensRet::error(e),
        }
    }
}

pub trait ClientCredentialsServiceComponent {
    type ClientCredentialsService: ClientCredentialsService;
    fn client_credentials_service(&self) -> &Self::ClientCredentialsService;
}

impl<
        T: AccessTokenRepositoryComponent + ClientRepositoryComponent + ResourceRepositoryComponent,
    > ClientCredentialsService for T
{
}

// Private functions
fn process_client_credentials(
    cmd: &ClientCredentialsCmd,
    access_token_repo: &AccessTokenRepository,
    client_repo: &ClientRepository,
    resource_repo: &ResourceRepository,
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

    // Resource
    let resource = resource_repo
        .find_by_id(&client.resource_id)
        .and_then(|v| {
            v.ok_or(
                ek::InvalidRequest(format!("Resource not found. ID => {}", client.resource_id))
                    .into(),
            )
        })?;

    // Validate scope
    let scope = cmd.scope
        .as_ref()
        .map_or(Vec::new(), |s| validate_scope_str(&s, &resource));

    // Access token
    let access_token = AccessToken::builder(&client_id, &resource.id)
        .scope(&scope)
        .build();
    access_token_repo.add(&access_token)?;

    Ok(TokensRet::builder(&access_token).build())
}

/// Validate the scope and return valid scope.
fn validate_scope_str(scope: &String, resource: &Resource) -> Vec<String> {
    let scope = scope.split(" ").map(|s| s.to_string()).collect();
    resource.filter_scope(&scope)
}
