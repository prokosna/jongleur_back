use chrono::prelude::*;
use time::Duration;

use util::generate_uid;
use domain::consts;
use domain::error::general as eg;
use self::eg::ResultExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshToken {
    #[serde(rename = "_id")] pub token: String,
    pub access_token_id: String,
    pub id_token_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub is_valid: bool,
    pub expires_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(access_token_id: &String, id_token_id: Option<&String>) -> eg::Result<RefreshToken> {
        let now = Utc::now();
        Ok(RefreshToken {
            token: generate_uid(64usize).chain_err(|| "generating uid failed")?,
            access_token_id: access_token_id.clone(),
            id_token_id: id_token_id.map(|x| x.clone()),
            created_at: now,
            is_valid: true,
            expires_at: now + Duration::seconds(consts::DEFAULT_REFRESH_TOKEN_MAX_AGE_SEC),
        })
    }

    pub fn is_valid(&self, refresh_token: &String) -> bool {
        &self.token == refresh_token && self.is_valid
    }
}
