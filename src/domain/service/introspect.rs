use self::ed::ErrorKind as ek;
use domain::error::domain as ed;
use domain::model::AccessToken;
use domain::repository::{AccessTokenRepository, AccessTokenRepositoryComponent, ClientRepository,
                         ClientRepositoryComponent, EndUserRepository, EndUserRepositoryComponent,
                         ResourceRepository, ResourceRepositoryComponent};
use domain::service::IntrospectRet;

pub struct IntrospectCmd {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub token: String,
    pub token_type_hint: Option<String>,
}

pub trait IntrospectService:
    AccessTokenRepositoryComponent
    + ClientRepositoryComponent
    + EndUserRepositoryComponent
    + ResourceRepositoryComponent
{
    fn introspect(&self, cmd: &IntrospectCmd) -> Result<IntrospectRet, ed::Error> {
        let access_token_repo = self.access_token_repository();
        let client_repo = self.client_repository();
        let end_user_repo = self.end_user_repository();
        let resource_repo = self.resource_repository();

        // Authorize the client
        let (client_id, client_secret) = cmd.client_id
            .as_ref()
            .and_then(|id| cmd.client_secret.as_ref().map(|secret| (id, secret)))
            .ok_or(ek::UnauthorizedClient("Invalid credentials.".to_string()))?;
        let client = client_repo.find_by_id(&client_id).and_then(|c| {
            c.ok_or(ek::UnauthorizedClient(format!("Client not found. ID => {}", client_id)).into())
        })?;
        if !client.is_authenticated_by_secret(&client_secret) {
            return Err(ek::UnauthorizedClient(format!(
                "Client not authorized. ID => {}",
                client_id
            )).into());
        }

        // Access token
        let access_token = access_token_repo.find_by_token(&cmd.token)?;
        if access_token.is_none() {
            return Ok(IntrospectRet::builder(false).build());
        }
        let access_token: AccessToken = access_token.unwrap();
        if !access_token.is_valid() {
            return Ok(IntrospectRet::builder(false).build());
        }

        // End user
        let end_user = if access_token.end_user_id.is_some() {
            let u = end_user_repo.find_by_id(&access_token.end_user_id.as_ref().unwrap())?;
            if u.is_none() {
                return Ok(IntrospectRet::builder(false).build());
            }
            u
        } else {
            None
        };

        // Resource
        let resource = resource_repo.find_by_id(&access_token.resource_id)?;
        if resource.is_none() {
            return Ok(IntrospectRet::builder(false).build());
        }

        // Scope
        let scope = resource
            .unwrap()
            .filter_scope(&access_token.scope)
            .join(" ");

        // Result
        Ok(IntrospectRet::builder(true)
            .scope(Some(scope))
            .client_id(Some(client_id.clone()))
            .username(end_user.as_ref().map(|v| v.name.to_string()))
            .exp(Some(access_token.expires_at().timestamp()))
            .iat(Some(access_token.created_at.timestamp()))
            .sub(end_user.as_ref().map(|v| v.id.to_string()))
            .aud(Some(client_id.clone()))
            .build())
    }
}

pub trait IntrospectServiceComponent {
    type IntrospectService: IntrospectService;
    fn introspect_service(&self) -> &Self::IntrospectService;
}

impl<
        T: AccessTokenRepositoryComponent
            + ClientRepositoryComponent
            + EndUserRepositoryComponent
            + ResourceRepositoryComponent,
    > IntrospectService for T
{
}
