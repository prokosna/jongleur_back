use std::marker::PhantomData;

use domain::consts;
use domain::repository::Repository;
use domain::model::{AccessToken, EndUser, EndUserClaims};
use domain::error::domain as ed;

pub struct UserinfoService<A, U>
where
    A: Repository<AccessToken>,
    U: Repository<EndUser>,
{
    _phantom1: PhantomData<A>,
    _phantom2: PhantomData<U>,
}

impl<A, U> UserinfoService<A, U>
where
    A: Repository<AccessToken>,
    U: Repository<EndUser>,
{
    pub fn new() -> Self {
        UserinfoService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn get_userinfo(
        &self,
        access_token: &String,
        access_token_repository: &A,
        end_user_repository: &U,
    ) -> Result<EndUserClaims, ed::Error> {
        let query = doc! {"token" => access_token};
        let token: AccessToken = access_token_repository.find(&query).and_then(|mut a| {
            a.pop().ok_or(
                ed::ErrorKind::UserinfoError("The access token was not found.".to_string()).into(),
            )
        })?;

        if !token.is_valid() {
            return Err(
                ed::ErrorKind::UserinfoError("The access token is expired.".to_string()).into(),
            );
        }

        let client_id = token.client_id;
        if let Some(end_user_id) = token.end_user_id {
            end_user_repository
                .find_by_key(&end_user_id)
                .and_then(|u| {
                    u.ok_or(
                        ed::ErrorKind::UserinfoError("The user does not exist.".to_string()).into(),
                    )
                })
                .map(|u| {
                    EndUserClaims::from_end_user(consts::ISSUER, &u, &client_id)
                })
        } else {
            Err(
                ed::ErrorKind::UserinfoError(
                    "The access token does not support openid. \
                     This may be created by OAuth 2.0 flow."
                        .to_string(),
                ).into(),
            )
        }
    }
}
