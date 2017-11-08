use chrono::prelude::*;
use time::Duration;

use config::AppConfig;
use domain::error::domain as ed;
use domain::model::{EndUser, ResponseType};
use util::generate_random_id;

/// `GrantType` is the type that represents `grant_type` in the context of
/// OAuth2 and OpenID Connect.
pub enum GrantType {
    Undefined(String),
    RefreshToken,
    ClientCredentials,
    Password,
    AuthorizationCode,
}

impl GrantType {
    pub fn new(grant_type: &str) -> Self {
        match grant_type.as_ref() {
            "refresh_token" => GrantType::RefreshToken,
            "client_credentials" => GrantType::ClientCredentials,
            "password" => GrantType::Password,
            "authorization_code" => GrantType::AuthorizationCode,
            _ => GrantType::Undefined(grant_type.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GrantStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "expired")]
    Expired,
}

/// `Grant` is the type that has a grant code in the context of
/// OAuth2 and OpenID Connect.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grant {
    pub id: String,
    pub end_user_id: String,
    pub client_id: String,
    pub resource_id: String,
    pub redirect_uri: String,
    pub code: String,
    pub expires_in: i64,
    pub scope: Vec<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub response_type: ResponseType,
    pub created_at: DateTime<Utc>,
    pub status: GrantStatus,
    pub is_deleted: bool,
}

impl Grant {
    /// Activate this grant.
    pub fn activate_with_end_user(&mut self, end_user: &EndUser) -> Result<(), ed::Error> {
        if self.end_user_id != end_user.id {
            return Err(
                ed::ErrorKind::AccessDenied("The granted user does not match.".to_string()).into(),
            );
        }

        if !self.is_valid() || self.status == GrantStatus::Activated {
            return Err(ed::ErrorKind::InvalidRequest(
                "This grant has been already used.".to_string(),
            ).into());
        }

        self.status = GrantStatus::Activated;
        Ok(())
    }

    /// Updates this grant with the timestamp.
    pub fn update_timestamp(&mut self, timestamp: DateTime<Utc>) -> () {
        self.created_at = timestamp;
    }

    /// Returns Some(String) if the response_type has code, otherwise None.
    pub fn get_code(&self) -> Option<String> {
        if self.response_type.has_code() {
            Some(self.code.to_string())
        } else {
            None
        }
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.created_at.clone() + Duration::seconds(self.expires_in)
    }

    pub fn is_valid(&self) -> bool {
        !(self.expires_at().timestamp() < Utc::now().timestamp()
            || self.status == GrantStatus::Expired)
    }

    pub fn builder(
        end_user_id: &String,
        client_id: &String,
        resource_id: &String,
        response_type: &ResponseType,
        redirect_uri: &String,
    ) -> GrantBuilder {
        GrantBuilder::new(
            end_user_id,
            client_id,
            resource_id,
            response_type,
            redirect_uri,
        )
    }
}

pub struct GrantBuilder {
    end_user_id: String,
    client_id: String,
    resource_id: String,
    redirect_uri: String,
    response_type: ResponseType,
    expires_in: i64,
    scope: Vec<String>,
    state: Option<String>,
    nonce: Option<String>,
}

impl GrantBuilder {
    fn new(
        end_user_id: &String,
        client_id: &String,
        resource_id: &String,
        response_type: &ResponseType,
        redirect_uri: &String,
    ) -> Self {
        GrantBuilder {
            end_user_id: end_user_id.clone(),
            client_id: client_id.clone(),
            resource_id: resource_id.clone(),
            redirect_uri: redirect_uri.clone(),
            response_type: response_type.clone(),
            expires_in: AppConfig::default_grant_max_age_sec(),
            scope: Vec::new(),
            state: None,
            nonce: None,
        }
    }

    pub fn expires_in(self, expires_in: i64) -> Self {
        GrantBuilder { expires_in, ..self }
    }

    pub fn scope(self, scope: &Vec<String>) -> Self {
        GrantBuilder {
            scope: scope.clone(),
            ..self
        }
    }

    pub fn state(self, state: &Option<String>) -> Self {
        GrantBuilder {
            state: state.clone(),
            ..self
        }
    }

    pub fn nonce(self, nonce: &Option<String>) -> Self {
        GrantBuilder {
            nonce: nonce.clone(),
            ..self
        }
    }

    pub fn build(self) -> Grant {
        Grant {
            id: generate_random_id(32usize),
            end_user_id: self.end_user_id,
            client_id: self.client_id,
            resource_id: self.resource_id,
            redirect_uri: self.redirect_uri,
            code: generate_random_id(64usize),
            expires_in: self.expires_in,
            scope: self.scope,
            state: self.state,
            nonce: self.nonce,
            response_type: self.response_type,
            created_at: Utc::now(),
            status: GrantStatus::Created,
            is_deleted: false,
        }
    }
}
