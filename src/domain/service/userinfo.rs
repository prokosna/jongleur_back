use self::ed::ErrorKind as ek;
use config::AppConfig;
use domain::error::domain as ed;
use domain::model::EndUserClaims;
use domain::repository::{AccessTokenRepository, AccessTokenRepositoryComponent, EndUserRepository,
                         EndUserRepositoryComponent};

pub struct UserinfoCmd {
    pub access_token: String,
}

pub trait UserinfoService: AccessTokenRepositoryComponent + EndUserRepositoryComponent {
    fn get_userinfo(&self, cmd: &UserinfoCmd) -> Result<EndUserClaims, ed::Error> {
        let access_token_repo = self.access_token_repository();
        let end_user_repo = self.end_user_repository();

        // Access token
        let access_token = access_token_repo
            .find_by_token(&cmd.access_token)
            .and_then(|v| {
                v.ok_or(ek::UserinfoError("The access token was not found.".to_string()).into())
            })?;

        if !access_token.is_valid() {
            return Err(ek::UserinfoError("The access token is expired.".to_string()).into());
        }

        if let Some(ref end_user_id) = access_token.end_user_id {
            end_user_repo
                .find_by_id(&end_user_id)
                .and_then(|v| {
                    v.ok_or(ek::UserinfoError("The user does not exist.".to_string()).into())
                })
                .map(|u| {
                    EndUserClaims::from_end_user(&AppConfig::issuer(), &u, &access_token.client_id)
                })
        } else {
            Err(ek::UserinfoError(
                "The access token does not support openid. \
                 This may be created by OAuth 2.0 flow."
                    .to_string(),
            ).into())
        }
    }
}

pub trait UserinfoServiceComponent {
    type UserinfoService: UserinfoService;
    fn userinfo_service(&self) -> &Self::UserinfoService;
}

impl<T: AccessTokenRepositoryComponent + EndUserRepositoryComponent> UserinfoService for T {}
