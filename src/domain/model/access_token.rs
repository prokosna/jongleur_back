use chrono::prelude::*;
use time::Duration;

use config::AppConfig;
use util::generate_random_id;

/// `AccessToken` is a type that represents an *access token*
/// in the context of OAuth2 and OpenID Connect.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    pub id: String,
    pub client_id: String,
    pub resource_id: String,
    pub token: String,
    pub expires_in: i64,
    pub created_at: DateTime<Utc>,
    pub scope: Vec<String>,
    pub end_user_id: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub is_deleted: bool,
}

impl AccessToken {
    /// Updates an access token with a new token.
    pub fn update(mut self) -> Self {
        self.token = generate_random_id(64usize);
        self.created_at = Utc::now();
        self
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.created_at.clone() + Duration::seconds(self.expires_in)
    }

    pub fn is_valid(&self) -> bool {
        !(self.expires_at().timestamp() < Utc::now().timestamp() || self.is_deleted)
    }

    pub fn builder(client_id: &String, resource_id: &String) -> AccessTokenBuilder {
        AccessTokenBuilder::new(client_id, resource_id)
    }
}

pub struct AccessTokenBuilder {
    client_id: String,
    resource_id: String,
    end_user_id: Option<String>,
    expires_in: i64,
    scope: Vec<String>,
    state: Option<String>,
    nonce: Option<String>,
}

impl AccessTokenBuilder {
    fn new(client_id: &String, resource_id: &String) -> Self {
        AccessTokenBuilder {
            client_id: client_id.clone(),
            resource_id: resource_id.clone(),
            expires_in: AppConfig::default_access_token_max_age_sec(),
            scope: Vec::new(),
            end_user_id: None,
            state: None,
            nonce: None,
        }
    }

    pub fn expires_in(self, expires_in: i64) -> Self {
        AccessTokenBuilder { expires_in, ..self }
    }

    pub fn scope(self, scope: &Vec<String>) -> Self {
        AccessTokenBuilder {
            scope: scope.clone(),
            ..self
        }
    }

    pub fn end_user_id(self, end_user_id: &Option<String>) -> Self {
        AccessTokenBuilder {
            end_user_id: end_user_id.clone(),
            ..self
        }
    }

    pub fn state(self, state: &Option<String>) -> Self {
        AccessTokenBuilder {
            state: state.clone(),
            ..self
        }
    }

    pub fn nonce(self, nonce: &Option<String>) -> Self {
        AccessTokenBuilder {
            nonce: nonce.clone(),
            ..self
        }
    }

    pub fn build(self) -> AccessToken {
        AccessToken {
            id: generate_random_id(32usize),
            client_id: self.client_id,
            resource_id: self.resource_id,
            token: generate_random_id(64usize),
            expires_in: self.expires_in,
            created_at: Utc::now(),
            scope: self.scope,
            end_user_id: self.end_user_id,
            state: self.state,
            nonce: self.nonce,
            is_deleted: false,
        }
    }
}
