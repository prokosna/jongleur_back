use chrono::prelude::*;
use uuid::Uuid;
use time::Duration;

use util::generate_uid;
use domain::consts;
use domain::error::general as eg;
use self::eg::ResultExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grant {
    #[serde(rename = "_id")] pub id: String,
    pub end_user_id: String,
    pub client_id: String,
    pub resource_id: String,
    pub redirect_uri: String,
    pub code: String,
    pub expires_in: i64,
    pub scope: Vec<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub response_types: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub is_valid: bool,
    pub expires_at: DateTime<Utc>,
}

impl Grant {
    pub fn is_valid(&self) -> bool {
        if self.expires_at.timestamp() < Utc::now().timestamp() {
            false
        } else {
            self.is_valid
        }
    }

    pub fn update_timestamp(&mut self, timestamp: DateTime<Utc>) -> () {
        self.created_at = timestamp;
        self.expires_at = timestamp + Duration::seconds(self.expires_in);
    }
}

pub struct GrantBuilder {
    end_user_id: String,
    client_id: String,
    resource_id: String,
    redirect_uri: String,
    expires_in: i64,
    scope: Vec<String>,
    state: Option<String>,
    nonce: Option<String>,
    response_types: Vec<String>,
}

impl GrantBuilder {
    pub fn new(
        end_user_id: String,
        client_id: String,
        resource_id: String,
        response_types: Vec<String>,
        redirect_uri: String,
    ) -> Self {
        GrantBuilder {
            end_user_id,
            client_id,
            resource_id,
            redirect_uri,
            expires_in: consts::DEFAULT_GRANT_MAX_AGE_SEC,
            scope: Vec::new(),
            state: None,
            nonce: None,
            response_types,
        }
    }

    pub fn expires_in(self, expires_in: i64) -> Self {
        GrantBuilder { expires_in, ..self }
    }

    pub fn scope(self, scope: Vec<String>) -> Self {
        GrantBuilder { scope, ..self }
    }

    pub fn state(self, state: Option<String>) -> Self {
        GrantBuilder { state, ..self }
    }

    pub fn nonce(self, nonce: Option<String>) -> Self {
        GrantBuilder { nonce, ..self }
    }

    pub fn build(self) -> self::eg::Result<Grant> {
        let created_at = Utc::now();
        let expires_at = created_at + Duration::seconds(self.expires_in);
        Ok(Grant {
            id: Uuid::new_v4().simple().to_string(),
            end_user_id: self.end_user_id,
            client_id: self.client_id,
            resource_id: self.resource_id,
            redirect_uri: self.redirect_uri,
            code: generate_uid(64usize).chain_err(|| "generating uid failed")?,
            expires_in: self.expires_in,
            scope: self.scope,
            state: self.state,
            nonce: self.nonce,
            response_types: self.response_types,
            created_at: Utc::now(),
            is_valid: false,
            expires_at,
        })
    }
}
