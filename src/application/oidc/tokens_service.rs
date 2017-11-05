use chrono::prelude::*;
use std::marker::PhantomData;

use application::oidc::{TokensParameters, TokensResponse, TokensResponseBuilder};
use domain::consts;
use domain::repository::Repository;
use domain::model::{AccessToken, AccessTokenBuilder, Client, EndUser, Grant, GrantType, IdToken,
                    IdTokenClaims, RefreshToken};
use domain::error::domain as ed;
use util::KeyStore;

pub struct TokensService<A, C, G, I, R, U>
where
    A: Repository<AccessToken>,
    C: Repository<Client>,
    G: Repository<Grant>,
    I: Repository<IdToken>,
    R: Repository<RefreshToken>,
    U: Repository<EndUser>,
{
    _phantom1: PhantomData<A>,
    _phantom2: PhantomData<C>,
    _phantom3: PhantomData<G>,
    _phantom4: PhantomData<I>,
    _phantom5: PhantomData<R>,
    _phantom6: PhantomData<U>,
}

impl<A, C, G, I, R, U> TokensService<A, C, G, I, R, U>
where
    A: Repository<AccessToken>,
    C: Repository<Client>,
    G: Repository<Grant>,
    I: Repository<IdToken>,
    R: Repository<RefreshToken>,
    U: Repository<EndUser>,
{
    pub fn new() -> Self {
        TokensService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
            _phantom4: PhantomData,
            _phantom5: PhantomData,
            _phantom6: PhantomData,
        }
    }

    pub fn get_tokens(
        &self,
        client_id: Option<String>,
        client_secret: Option<String>,
        tokens_parameters: &TokensParameters,
        client_repository: &C,
        end_user_repository: &U,
        grant_repository: &G,
        access_token_repository: &A,
        refresh_token_repository: &R,
        id_token_repository: &I,
    ) -> Result<TokensResponse, ed::Error> {
        let (client_id, client_secret) = client_id
            .and_then(|id| client_secret.map(|secret| (id, secret)))
            .ok_or(ed::ErrorKind::UnauthorizedClient(
                "Invalid credentials.".to_string(),
            ))?;

        let client = client_repository.find_by_key(&client_id).and_then(|c| {
            c.ok_or(
                ed::ErrorKind::UnauthorizedClient("The client was not found.".to_string()).into(),
            )
        })?;
        if !client.authenticate_by_secret(&client_secret) {
            return Err(
                ed::ErrorKind::UnauthorizedClient("The client is not authorized.".to_string())
                    .into(),
            );
        }

        let grant_type = GrantType::new(&tokens_parameters.grant_type);

        match grant_type {
            GrantType::AuthorizationCode => {
                let mut grant = match tokens_parameters.code {
                    Some(ref c) => {
                        let query = doc! {"code" => c};
                        grant_repository.find(&query).and_then(|mut g| {
                            g.pop().ok_or(
                                ed::ErrorKind::InvalidGrant("The grant is not valid.".to_string())
                                    .into(),
                            )
                        })?
                    }
                    None => {
                        return Err(
                            ed::ErrorKind::InvalidGrant("Invalid grant code.".to_string()).into(),
                        );
                    }
                };

                if !grant.is_valid || grant.client_id != client.id {
                    return Err(
                        ed::ErrorKind::InvalidGrant("Invalid grant code.".to_string()).into(),
                    );
                }

                let elapsed_sec = Utc::now().signed_duration_since(grant.created_at);
                if elapsed_sec.num_seconds() - grant.expires_in > 0 {
                    return Err(
                        ed::ErrorKind::InvalidGrant("Grant has been already expired.".to_string())
                            .into(),
                    );
                }

                grant.is_valid = false;
                grant.expires_in = Utc::now().timestamp();
                grant_repository.update(&grant)?;

                let mut access_token_builder =
                    AccessTokenBuilder::new(grant.client_id.clone(), grant.resource_id.clone());
                access_token_builder = access_token_builder
                    .end_user_id(Some(grant.end_user_id.clone()))
                    .scope(grant.scope.clone())
                    .nonce(grant.nonce.clone());
                let access_token = access_token_builder.build().unwrap();
                access_token_repository.insert(&access_token)?;

                // TODO: scope should be checked through both an auth request and a token request
                if grant.scope.iter().any(|x| x == "openid") {
                    // OpenID flow
                    let end_user = end_user_repository
                        .find_by_key(&grant.end_user_id)
                        .and_then(|u| {
                            u.ok_or(
                                ed::ErrorKind::InvalidGrant("The user does not exist.".to_string())
                                    .into(),
                            )
                        })?;
                    let id_token =
                        IdTokenClaims::from_end_user(consts::ISSUER, &end_user, &client.id)
                            .nonce(grant.nonce.clone())
                            .publish(&KeyStore::jwt_private_key())?;
                    id_token_repository.insert(&id_token)?;

                    let refresh_token =
                        RefreshToken::new(&access_token.id, Some(&id_token.id)).unwrap();
                    refresh_token_repository.insert(&refresh_token)?;

                    let res =
                        TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                            .refresh_token(&refresh_token.token)
                            .id_token(&id_token.token)
                            .build();
                    Ok(res)
                } else {
                    // OAuth 2.0 flow
                    let refresh_token = RefreshToken::new(&access_token.id, None).unwrap();
                    refresh_token_repository.insert(&refresh_token)?;

                    let res =
                        TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                            .refresh_token(&refresh_token.token)
                            .build();
                    Ok(res)
                }
            }
            GrantType::RefreshToken => {
                let token = tokens_parameters.refresh_token.as_ref().ok_or(
                    ed::ErrorKind::InvalidRequest("Invalid refresh_token.".to_string()),
                )?;

                let refresh_token = refresh_token_repository.find_by_key(&token).and_then(|r| {
                    r.ok_or(
                        ed::ErrorKind::InvalidRequest("Invalid refresh_token.".to_string()).into(),
                    )
                })?;
                if !refresh_token.is_valid(&token) {
                    return Err(
                        ed::ErrorKind::InvalidRequest("Invalid refresh_token.".to_string()).into(),
                    );
                }

                let access_token = access_token_repository
                    .find_by_key(&refresh_token.access_token_id)
                    .and_then(|t| {
                        t.ok_or(
                            ed::ErrorKind::InvalidRequest("Invalid refresh_token.".to_string())
                                .into(),
                        )
                    })?;
                let new_access_token = access_token.update().unwrap();
                access_token_repository.update(&new_access_token)?;

                if tokens_parameters
                    .scope
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .split(" ")
                    .any(|x| x == "openid")
                    && refresh_token.id_token_id.is_some()
                {
                    let id_token = id_token_repository
                        .find_by_key(&refresh_token.id_token_id.unwrap())
                        .and_then(|t| {
                            t.ok_or(
                                ed::ErrorKind::InvalidRequest("Invalid refresh_token.".to_string())
                                    .into(),
                            )
                        })?;

                    let claims = id_token.extract_claims(&KeyStore::jwt_public_key())?;

                    let end_user = end_user_repository.find_by_key(&claims.sub).and_then(|u| {
                        u.ok_or(
                            ed::ErrorKind::InvalidRequest("The user does not exist.".to_string())
                                .into(),
                        )
                    })?;

                    let mut new_id_token =
                        IdTokenClaims::from_end_user(consts::ISSUER, &end_user, &client.id)
                            .auth_time(claims.auth_time.clone())
                            .azp(claims.azp.clone())
                            .publish(&KeyStore::jwt_private_key())?;
                    new_id_token.id = id_token.id.clone();
                    id_token_repository.update(&new_id_token)?;

                    let res =
                        TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                            .refresh_token(&refresh_token.token)
                            .id_token(&new_id_token.token)
                            .build();
                    Ok(res)
                } else {
                    let res =
                        TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                            .refresh_token(&refresh_token.token)
                            .build();
                    Ok(res)
                }
            }
            GrantType::ClientCredentials => {
                // TODO: Ref Resource
                let scope: Vec<String> = tokens_parameters
                    .scope
                    .clone()
                    .unwrap_or("".to_string())
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect();

                let mut access_token_builder =
                    AccessTokenBuilder::new(client.id.clone(), client.resource_id.clone());
                access_token_builder = access_token_builder.scope(scope.clone());
                let access_token = access_token_builder.build().unwrap();
                access_token_repository.insert(&access_token)?;

                let refresh_token = RefreshToken::new(&access_token.id, None).unwrap();
                refresh_token_repository.insert(&refresh_token)?;

                let res = TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                    .refresh_token(&refresh_token.token)
                    .build();
                Ok(res)
            }
            GrantType::Password => {
                let (username, password) = tokens_parameters
                    .username
                    .as_ref()
                    .and_then(|u| tokens_parameters.password.as_ref().map(|p| (u, p)))
                    .ok_or(ed::ErrorKind::InvalidRequest(
                        "Both username and password are required.".to_string(),
                    ))?;

                let query = doc! {"name" => username};
                let mut end_user = end_user_repository.find(&query).and_then(|mut u| {
                    u.pop().ok_or(
                        ed::ErrorKind::AccessDenied("username or password is invalid.".to_string())
                            .into(),
                    )
                })?;
                if !end_user.authenticate(&password) {
                    return Err(
                        ed::ErrorKind::AccessDenied("username or password is invalid.".to_string())
                            .into(),
                    );
                }
                end_user_repository.update(&end_user)?;

                // TODO: Ref Resource
                let scope: Vec<String> = tokens_parameters
                    .scope
                    .clone()
                    .unwrap_or("".to_string())
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect();

                let mut access_token_builder =
                    AccessTokenBuilder::new(client.id.clone(), client.resource_id.clone());
                access_token_builder = access_token_builder.scope(scope.clone());
                let access_token = access_token_builder.build().unwrap();
                access_token_repository.insert(&access_token)?;

                // Password flow may be not supported in OIDC context.
                //
                // if scope.iter().any(|x| x == "openid") {
                //   // OpenID flow
                //   let id_token =
                //     IdTokenClaims::from_end_user(consts::ISSUER, &end_user, &client.id)
                //        .publish(&KeyStore::jwt_private_key())?;
                //   id_token_repository.insert(&id_token)?;
                //
                //   let refresh_token =
                //     RefreshToken::new(&access_token.id, Some(&id_token.id)).unwrap();
                //   refresh_token_repository.insert(&refresh_token)?;
                //
                //   let res =
                //     TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                //       .refresh_token(&refresh_token.token)
                //       .id_token(&id_token.token)
                //       .build();
                //   Ok(res)
                // } else {

                let refresh_token = RefreshToken::new(&access_token.id, None).unwrap();
                refresh_token_repository.insert(&refresh_token)?;

                let res = TokensResponseBuilder::new(&access_token.token, access_token.expires_in)
                    .refresh_token(&refresh_token.token)
                    .build();
                Ok(res)
            }
            GrantType::Undefined(ref raw) => Err(
                ed::ErrorKind::UnsupportedGrantType(format!("Invalid grant type: {}", raw)).into(),
            ),
        }
    }
}
