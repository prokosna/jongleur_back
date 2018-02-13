use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

use config::AppConfig;
use domain::model::{Client, EndUser};
use domain::error::domain as ed;
use util::generate_random_id;
use self::ed::ResultExt;

/// `IdToken` is the type contains `id_token` in the context of
/// OAuth2 and OpenID Connect.
#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
    pub id: String,
    pub end_user_id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl IdToken {
    /// Returns `true` if the IdToken is valid.
    pub fn is_valid(&self, key: &Vec<u8>) -> bool {
        if let Ok(decoded) =
            decode::<IdTokenClaims>(&self.token, key.as_ref(), &Validation::default())
        {
            !(decoded.claims.exp < Utc::now().timestamp() || self.is_deleted)
        } else {
            false
        }
    }

    /// Extracts IdTokenClaims from IdToken using a public key of an authorization server.
    pub fn extract_claims(&self, key: &Vec<u8>) -> Result<IdTokenClaims, ed::Error> {
        decode::<IdTokenClaims>(&self.token, key.as_ref(), &Validation::default())
            .map(|data| data.claims)
            .chain_err(|| {
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when decoding the id token.".to_string(),
                )
            })
    }

    /// Refreshes a token of the IdToken.
    pub fn update(
        self,
        client: &Client,
        end_user: &EndUser,
        public_key: &Vec<u8>,
        private_key: &Vec<u8>,
    ) -> Result<Self, ed::Error> {
        let claims = self.extract_claims(public_key.as_ref())?;
        let mut new_id_token =
            IdTokenClaims::from_end_user(&AppConfig::issuer(), &end_user, &client.id)
                .nonce(&claims.nonce)
                .acr(&claims.acr)
                .amr(&claims.amr)
                .azp(&claims.azp)
                .publish(private_key.as_ref())?;
        new_id_token.id = self.id;
        new_id_token.is_deleted = self.is_deleted;
        Ok(new_id_token)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            exp: now.timestamp() + AppConfig::default_id_token_max_age_sec(),
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
            exp: now.timestamp() + AppConfig::default_id_token_max_age_sec(),
            iat: now.timestamp(),
            auth_time: end_user.authenticated_at.as_ref().map(|t| t.timestamp()),
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

    pub fn auth_time(self, auth_time: &Option<i64>) -> Self {
        IdTokenClaims {
            auth_time: auth_time.clone(),
            ..self
        }
    }

    pub fn nonce(self, nonce: &Option<String>) -> Self {
        IdTokenClaims {
            nonce: nonce.clone(),
            ..self
        }
    }

    pub fn acr(self, acr: &Option<String>) -> Self {
        IdTokenClaims {
            acr: acr.clone(),
            ..self
        }
    }

    pub fn amr(self, amr: &Option<String>) -> Self {
        IdTokenClaims {
            amr: amr.clone(),
            ..self
        }
    }

    pub fn azp(self, azp: &Option<String>) -> Self {
        IdTokenClaims {
            azp: azp.clone(),
            ..self
        }
    }

    pub fn publish(self, key: &Vec<u8>) -> Result<IdToken, ed::Error> {
        let expires_at = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.exp, 0), Utc);
        let mut header = Header::default();
        header.alg = Algorithm::RS256;
        let id_token: String = encode(&header, &self, key.as_ref()).chain_err(|| {
            ed::ErrorKind::ServerError("Encoding claims to JWT failed.".to_string())
        })?;
        Ok(IdToken {
            id: generate_random_id(32usize),
            end_user_id: self.sub.clone(),
            token: id_token,
            created_at: Utc::now(),
            expires_at,
            is_deleted: false,
        })
    }
}
