use domain::model::{AccessToken, IdToken, RefreshToken};
use domain::error::domain as ed;

#[derive(Serialize)]
#[serde(untagged)]
pub enum TokensRetKind {
    Token {
        access_token: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        refresh_token: Option<String>,
        token_type: String,
        expires_in: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        id_token: Option<String>,
    },
    Error {
        error: String,
        error_description: String,
        #[serde(skip_serializing)]
        _cause: ed::Error,
    },
}

pub struct TokensRet {
    pub kind: TokensRetKind,
}

impl TokensRet {
    pub fn error(error: ed::Error) -> Self {
        TokensRet {
            kind: TokensRetKind::Error {
                error: error.description().to_string(),
                error_description: error.to_string(),
                _cause: error,
            },
        }
    }

    pub fn builder(access_token: &AccessToken) -> AuthorizeRetBuilder {
        AuthorizeRetBuilder::new(access_token)
    }
}

pub struct AuthorizeRetBuilder {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
    id_token: Option<String>,
}

impl AuthorizeRetBuilder {
    fn new(access_token: &AccessToken) -> Self {
        AuthorizeRetBuilder {
            access_token: access_token.token.clone(),
            expires_in: access_token.expires_in,
            refresh_token: None,
            id_token: None,
        }
    }

    pub fn id_token(self, id_token: &Option<IdToken>) -> Self {
        AuthorizeRetBuilder {
            id_token: id_token.as_ref().map(|v| v.token.clone()),
            ..self
        }
    }

    pub fn refresh_token(self, refresh_token: &Option<RefreshToken>) -> Self {
        AuthorizeRetBuilder {
            refresh_token: refresh_token.as_ref().map(|v| v.token.clone()),
            ..self
        }
    }

    pub fn build(self) -> TokensRet {
        let kind = TokensRetKind::Token {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.expires_in,
            id_token: self.id_token,
        };

        TokensRet { kind }
    }
}
