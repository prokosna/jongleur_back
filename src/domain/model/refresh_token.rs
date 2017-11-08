use chrono::prelude::*;
use time::Duration;

use config::AppConfig;
use util::generate_random_id;

/// `RefreshToken` is the type that contains a `refresh_token`
/// in the context of OAuth2 and OpenID Connect.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshToken {
    pub token: String,
    pub access_token_id: String,
    pub id_token_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl RefreshToken {
    pub fn new(access_token_id: &String, id_token_id: &Option<String>) -> RefreshToken {
        let now = Utc::now();
        RefreshToken {
            token: generate_random_id(64usize),
            access_token_id: access_token_id.clone(),
            id_token_id: id_token_id.clone(),
            created_at: now,
            expires_at: now + Duration::seconds(AppConfig::default_refresh_token_max_age_sec()),
            is_deleted: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        !(self.expires_at.timestamp() < Utc::now().timestamp() || self.is_deleted)
    }
}
