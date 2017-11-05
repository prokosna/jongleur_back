use chrono::prelude::*;
use uuid::Uuid;
use time::Duration;

use util::generate_uid;
use domain::consts;
use domain::error::general as eg;
use self::eg::ResultExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    #[serde(rename = "_id")] pub id: String,
    pub client_id: String,
    pub resource_id: String,
    pub token: String,
    pub expires_in: i64,
    pub created_at: DateTime<Utc>,
    pub scope: Vec<String>,
    pub end_user_id: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub is_valid: bool,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn update(&self) -> eg::Result<AccessToken> {
        let mut new_token = self.clone();
        let created_at = Utc::now();
        let expires_at = created_at.clone() + Duration::seconds(self.expires_in);
        new_token.token = generate_uid(64usize).chain_err(|| "generating uid failed")?;
        new_token.created_at = created_at;
        new_token.expires_at = expires_at;
        Ok(new_token)
    }

    pub fn is_valid(&self) -> bool {
        if self.expires_at.timestamp() < Utc::now().timestamp() {
            false
        } else {
            self.is_valid
        }
    }
}

pub struct AccessTokenBuilder {
    client_id: String,
    resource_id: String,
    expires_in: i64,
    scope: Vec<String>,
    end_user_id: Option<String>,
    state: Option<String>,
    nonce: Option<String>,
}

impl AccessTokenBuilder {
    pub fn new(client_id: String, resource_id: String) -> Self {
        AccessTokenBuilder {
            client_id,
            resource_id,
            expires_in: consts::DEFAULT_ACCESS_TOKEN_MAX_AGE_SEC,
            scope: Vec::new(),
            end_user_id: None,
            state: None,
            nonce: None,
        }
    }

    pub fn expires_in(self, expires_in: i64) -> Self {
        AccessTokenBuilder { expires_in, ..self }
    }

    pub fn scope(self, scope: Vec<String>) -> Self {
        AccessTokenBuilder { scope, ..self }
    }

    pub fn end_user_id(self, end_user_id: Option<String>) -> Self {
        AccessTokenBuilder {
            end_user_id,
            ..self
        }
    }

    pub fn state(self, state: Option<String>) -> Self {
        AccessTokenBuilder { state, ..self }
    }

    pub fn nonce(self, nonce: Option<String>) -> Self {
        AccessTokenBuilder { nonce, ..self }
    }

    pub fn build(self) -> self::eg::Result<AccessToken> {
        let created_at = Utc::now();
        let expires_at = created_at.clone() + Duration::seconds(self.expires_in);
        Ok(AccessToken {
            id: Uuid::new_v4().simple().to_string(),
            client_id: self.client_id,
            resource_id: self.resource_id,
            token: generate_uid(64usize).chain_err(|| "generating uid failed")?,
            expires_in: self.expires_in,
            created_at: Utc::now(),
            scope: self.scope,
            end_user_id: self.end_user_id,
            state: self.state,
            nonce: self.nonce,
            is_valid: true,
            expires_at,
        })
    }
}
