use chrono::prelude::*;
use uuid::Uuid;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

use domain::consts;
use domain::model::end_user::EndUser;
use domain::error::domain as ed;
use self::ed::ResultExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
    #[serde(rename = "_id")] pub id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub is_valid: bool,
    pub expires_at: DateTime<Utc>,
}

impl IdToken {
    pub fn is_valid(&self, key: &String) -> bool {
        if let Ok(decoded) =
            decode::<IdTokenClaims>(&self.token, key.as_ref(), &Validation::default())
        {
            if decoded.claims.exp < Utc::now().timestamp() {
                false
            } else {
                self.is_valid
            }
        } else {
            false
        }
    }

    pub fn extract_claims(&self, key: &Vec<u8>) -> Result<IdTokenClaims, ed::Error> {
        decode::<IdTokenClaims>(&self.token, key.as_ref(), &Validation::default())
            .map(|data| data.claims)
            .chain_err(|| {
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when decoding the id token.".to_string(),
                )
            })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdTokenClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub auth_time: Option<i64>,
    pub nonce: Option<String>,
    pub acr: Option<String>,
    pub amr: Option<String>,
    pub azp: Option<String>,
}

impl IdTokenClaims {
    pub fn new(issuer: &String, end_user_id: &String, client_id: &String) -> Self {
        let now = Utc::now();
        IdTokenClaims {
            iss: issuer.clone(),
            sub: end_user_id.clone(),
            aud: client_id.clone(),
            exp: now.timestamp() + consts::DEFAULT_ID_TOKEN_MAX_AGE_SEC,
            iat: now.timestamp(),
            auth_time: None,
            nonce: None,
            acr: None,
            amr: None,
            azp: None,
        }
    }

    pub fn from_end_user(issuer: &str, end_user: &EndUser, client_id: &String) -> Self {
        let now = Utc::now();
        IdTokenClaims {
            iss: issuer.to_string(),
            sub: end_user.id.clone(),
            aud: client_id.clone(),
            exp: now.timestamp() + consts::DEFAULT_ID_TOKEN_MAX_AGE_SEC,
            iat: now.timestamp(),
            auth_time: end_user
                .last_authenticated_at
                .as_ref()
                .map(|t| t.timestamp()),
            nonce: None,
            acr: None,
            amr: None,
            azp: None,
        }
    }

    pub fn exp(self, exp: i64) -> Self {
        IdTokenClaims { exp, ..self }
    }

    pub fn iat(self, iat: i64) -> Self {
        IdTokenClaims { iat, ..self }
    }

    pub fn auth_time(self, auth_time: Option<i64>) -> Self {
        IdTokenClaims { auth_time, ..self }
    }

    pub fn nonce(self, nonce: Option<String>) -> Self {
        IdTokenClaims { nonce, ..self }
    }

    pub fn acr(self, acr: Option<String>) -> Self {
        IdTokenClaims { acr, ..self }
    }

    pub fn amr(self, amr: Option<String>) -> Self {
        IdTokenClaims { amr, ..self }
    }

    pub fn azp(self, azp: Option<String>) -> Self {
        IdTokenClaims { azp, ..self }
    }

    pub fn publish(self, key: &Vec<u8>) -> ed::Result<IdToken> {
        let expires_at = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.exp, 0), Utc);
        let mut header = Header::default();
        header.alg = Algorithm::RS256;
        let id_token: String = encode(&header, &self, key.as_ref()).chain_err(|| {
            ed::ErrorKind::ServerError("Encoding claims to JWT failed.".to_string())
        })?;
        Ok(IdToken {
            id: Uuid::new_v4().simple().to_string(),
            token: id_token,
            created_at: Utc::now(),
            is_valid: true,
            expires_at,
        })
    }
}
