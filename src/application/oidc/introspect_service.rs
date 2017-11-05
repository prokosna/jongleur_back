use std::marker::PhantomData;

use domain::repository::Repository;
use domain::model::{AccessToken, Client, EndUser, Resource};
use domain::error::domain as ed;
use application::oidc::{IntrospectParameters, IntrospectResponse, IntrospectResponseBuilder};

pub struct IntrospectService<A, C, U, V>
where
    A: Repository<AccessToken>,
    C: Repository<Client>,
    U: Repository<EndUser>,
    V: Repository<Resource>,
{
    _phantom1: PhantomData<A>,
    _phantom2: PhantomData<C>,
    _phantom3: PhantomData<U>,
    _phantom4: PhantomData<V>,
}

impl<A, C, U, V> IntrospectService<A, C, U, V>
where
    A: Repository<AccessToken>,
    C: Repository<Client>,
    U: Repository<EndUser>,
    V: Repository<Resource>,
{
    pub fn new() -> Self {
        IntrospectService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
            _phantom4: PhantomData,
        }
    }

    pub fn introspect_token(
        &self,
        client_id: Option<String>,
        client_secret: Option<String>,
        introspect_parameters: &IntrospectParameters,
        access_token_repository: &A,
        client_repository: &C,
        end_user_repository: &U,
        resource_repository: &V,
    ) -> Result<IntrospectResponse, ed::Error> {
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

        // Access Token
        let token = &introspect_parameters.token;
        let query = doc! {"token" => token};
        let access_token = access_token_repository
            .find(&query)
            .and_then(|mut a| a.pop().ok_or("".into()));
        if let Err(_) = access_token {
            return Ok(IntrospectResponseBuilder::new(false).build());
        }
        let access_token = access_token.unwrap();

        if !access_token.is_valid() {
            return Ok(IntrospectResponseBuilder::new(false).build());
        }


        let end_user = access_token
            .end_user_id
            .as_ref()
            .and_then(|id| end_user_repository.find_by_key(&id).unwrap_or(None));
        let resource = resource_repository
            .find_by_key(&access_token.resource_id)
            .unwrap_or(None);
        let is_valid =
            if resource.is_none() || (access_token.end_user_id.is_some() && end_user.is_none()) {
                false
            } else {
                true
            };

        // Response
        let resource = resource.unwrap();
        let mut introspect_response_builder = IntrospectResponseBuilder::new(is_valid);
        if is_valid {
            let scope = access_token
                .scope
                .iter()
                .filter(|s1| resource.scope.iter().any(|s2| s1 == &&s2.name))
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            introspect_response_builder = introspect_response_builder
                .scope(Some(scope))
                .client_id(Some(client_id.clone()))
                .username(end_user.as_ref().map(|u| u.name.to_string()))
                .exp(Some(access_token.expires_at.timestamp()))
                .iat(Some(access_token.created_at.timestamp()))
                .sub(end_user.as_ref().map(|u| u.id.to_string()))
                .aud(Some(client_id.clone()));
        }
        Ok(introspect_response_builder.build())
    }
}
