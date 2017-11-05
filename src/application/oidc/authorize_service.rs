use chrono::prelude::*;
use std::marker::PhantomData;

use application::oidc::{AuthParameters, AuthResponse, AuthResponseBuilder};
use domain::consts;
use domain::session::Store;
use domain::repository::Repository;
use domain::model::{AcceptedClient, AccessToken, AccessTokenBuilder, Client, EndUser, FlowType,
                    Grant, GrantBuilder, IdToken, IdTokenClaims, RefreshToken, Resource,
                    ResponseType};
use domain::error::domain as ed;
use util::KeyStore;

pub struct AuthorizeService<A, C, G, I, R, S, U, V>
where
    A: Repository<AccessToken>,
    C: Repository<Client>,
    G: Repository<Grant>,
    I: Repository<IdToken>,
    R: Repository<RefreshToken>,
    S: Store,
    U: Repository<EndUser>,
    V: Repository<Resource>,
{
    _phantom1: PhantomData<A>,
    _phantom2: PhantomData<C>,
    _phantom3: PhantomData<G>,
    _phantom4: PhantomData<I>,
    _phantom5: PhantomData<S>,
    _phantom6: PhantomData<R>,
    _phantom7: PhantomData<U>,
    _phantom8: PhantomData<V>,
}

impl<A, C, G, I, R, S, U, V> AuthorizeService<A, C, G, I, R, S, U, V>
where
    A: Repository<AccessToken>,
    C: Repository<Client>,
    G: Repository<Grant>,
    I: Repository<IdToken>,
    R: Repository<RefreshToken>,
    S: Store,
    U: Repository<EndUser>,
    V: Repository<Resource>,
{
    pub fn new() -> Self {
        AuthorizeService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
            _phantom4: PhantomData,
            _phantom5: PhantomData,
            _phantom6: PhantomData,
            _phantom7: PhantomData,
            _phantom8: PhantomData,
        }
    }

    /// Check and return a valid redirect_uri and a Client instance
    pub fn pre_process_auth(
        &self,
        auth_parameters: &AuthParameters,
        client_repository: &C,
    ) -> Result<(String, Client), ed::Error> {
        let client = client_repository
            .find_by_key(&auth_parameters.client_id)
            .and_then(|c| c.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        let redirect_uri = Self::validate_redirect_uri(&auth_parameters.redirect_uri, &client)?;
        Ok((redirect_uri, client))
    }

    pub fn process_auth(
        &self,
        sid: &String,
        client: &Client,
        redirect_uri: &String,
        auth_parameters: &AuthParameters,
        store: &S,
        end_user_repository: &U,
        grant_repository: &G,
        access_token_repository: &A,
        resource_repository: &V,
    ) -> Result<AuthResponse, ed::Error> {
        let response_type = ResponseType::from_str(&auth_parameters.response_type);
        if let FlowType::Undefined = response_type.flow_type() {
            return Err(ed::ErrorKind::InvalidRequest("Invalid response_type.".to_string()).into());
        }

        let resource_id = &client.resource_id;
        let resource = resource_repository
            .find_by_key(&resource_id)
            .and_then(|r| r.ok_or(ed::ErrorKind::EntryNotFound.into()))?;

        // All scope must be included by the scope of Resource
        let scope = auth_parameters
            .scope
            .as_ref()
            .map(|s| {
                s.split(" ")
                    .map(|x| x.to_string())
                    .filter(|s| resource.scope.iter().any(|scope| s == &scope.name))
                    .collect()
            })
            .unwrap_or(Vec::new());

        let end_user_id = store.get(&sid, consts::END_USER_SESS_ID_FIELD)?;
        let mut end_user = end_user_repository
            .find_by_key(&end_user_id)
            .and_then(|u| u.ok_or(ed::ErrorKind::EntryNotFound.into()))?;

        let mut grant_builder = GrantBuilder::new(
            end_user.id.clone(),
            client.id.clone(),
            resource_id.clone(),
            response_type.to_vec(),
            auth_parameters.redirect_uri.clone(),
        );
        grant_builder = grant_builder
            .scope(scope.clone())
            .state(auth_parameters.state.clone());
        let mut grant = grant_builder.build().unwrap();
        grant_repository.insert(&grant)?;

        // check if the end user accepts the client and the scope
        if end_user.accepted_clients.iter().any(|c| {
            c.client_id == client.id && !scope.iter().any(|s1| !c.scope.iter().any(|s2| s1 == s2))
        }) {
            // the user has already accepted the client and the scope
            self.process_grant(
                &redirect_uri,
                &mut grant,
                &mut end_user,
                &end_user_repository,
                &grant_repository,
                &access_token_repository,
            )
        } else {
            // request acceptance for the client
            Ok(AuthResponse::require_acceptance(
                &grant.id,
                &resource
                    .scope
                    .iter()
                    .filter(|x| scope.iter().any(|s| s == &x.name))
                    .map(|x| x.clone())
                    .collect(),
            ))
        }
    }

    pub fn pre_process_accept(
        &self,
        grant_id: &String,
        client_repository: &C,
        grant_repository: &G,
    ) -> Result<(String, Client, Grant), ed::Error> {
        let grant = grant_repository.find_by_key(&grant_id).and_then(|g| {
            g.ok_or(ed::ErrorKind::InvalidRequest("Invalid grant_id.".to_string()).into())
        })?;
        let client = client_repository
            .find_by_key(&grant.client_id)
            .and_then(|c| c.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        let redirect_uri = Self::validate_redirect_uri(&grant.redirect_uri, &client)?;
        Ok((redirect_uri, client, grant))
    }

    pub fn process_accept(
        &self,
        sid: &String,
        action: &String,
        redirect_uri: &String,
        mut grant: Grant,
        store: &S,
        end_user_repository: &U,
        grant_repository: &G,
        access_token_repository: &A,
    ) -> Result<AuthResponse, ed::Error> {
        if action != consts::ACTION_ACCEPT {
            return Err(
                ed::ErrorKind::AccessDenied("The user rejected the request.".to_string()).into(),
            );
        }

        let end_user_id = store.get(&sid, consts::END_USER_SESS_ID_FIELD)?;
        let mut end_user = end_user_repository
            .find_by_key(&end_user_id)
            .and_then(|u| u.ok_or(ed::ErrorKind::EntryNotFound.into()))?;

        self.process_grant(
            &redirect_uri,
            &mut grant,
            &mut end_user,
            &end_user_repository,
            &grant_repository,
            &access_token_repository,
        )
    }

    fn process_grant(
        &self,
        redirect_uri: &String,
        grant: &mut Grant,
        end_user: &mut EndUser,
        end_user_repository: &U,
        grant_repository: &G,
        access_token_repository: &A,
    ) -> Result<AuthResponse, ed::Error> {
        if grant.end_user_id != end_user.id {
            return Err(
                ed::ErrorKind::AccessDenied(
                    "The granted user does not match with the accesing user.".to_string(),
                ).into(),
            );
        }

        // is_valid must be false here because it is set true after the following process
        if grant.is_valid {
            return Err(
                ed::ErrorKind::InvalidRequest("Specified grant has been already used.".to_string())
                    .into(),
            );
        }

        // add the client and the scope to the end-user as an accepted client
        let accepted_client = AcceptedClient {
            client_id: grant.client_id.clone(),
            scope: grant.scope.clone(),
        };
        end_user.add_accepted_client(accepted_client);
        end_user_repository.update(&end_user)?;

        // set true to grant.is_valid if response_type has 'code'
        let response_type = ResponseType::new(&grant.response_types);
        if response_type.has_code() {
            grant.is_valid = true;
            grant.update_timestamp(Utc::now());
            grant_repository.update(&grant)?;
        }

        // create auth_response
        let mut auth_response_builder = AuthResponseBuilder::new(redirect_uri, &grant.state);
        if response_type.has_code() {
            auth_response_builder = auth_response_builder.code(&grant.code);
        }
        if response_type.has_token() {
            let mut access_token_builder =
                AccessTokenBuilder::new(grant.client_id.clone(), grant.resource_id.clone());
            access_token_builder = access_token_builder
                .end_user_id(Some(grant.end_user_id.clone()))
                .state(grant.state.clone())
                .scope(grant.scope.clone());
            let access_token: AccessToken = access_token_builder.build().unwrap();
            access_token_repository.insert(&access_token)?;
            auth_response_builder =
                auth_response_builder.access_token(&access_token.token, access_token.expires_in);
        }
        if response_type.has_id_token() {
            let id_token_claim =
                IdTokenClaims::from_end_user(consts::ISSUER, &end_user, &grant.client_id);
            let id_token: IdToken = id_token_claim.publish(&KeyStore::jwt_private_key())?;
            auth_response_builder = auth_response_builder.id_token(&id_token.token);
        }

        match auth_response_builder.build() {
            Some(auth_response) => Ok(auth_response),
            None => Err(
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when creating an auth_response.".to_string(),
                ).into(),
            ),
        }
    }

    fn validate_redirect_uri(redirect_uri: &String, client: &Client) -> Result<String, ed::Error> {
        if !client.redirect_uris.iter().any(|uri| uri == redirect_uri) {
            return Err(ed::ErrorKind::InvalidRequest("Invalid redirect_uri.".to_string()).into());
        }
        Ok(redirect_uri.clone())
    }
}
