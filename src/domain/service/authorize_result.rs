use domain::error::domain as ed;
use domain::model::{AccessToken, IdToken, Scope};

#[derive(Serialize)]
#[serde(untagged)]
pub enum AuthorizeRetKind {
    Code {
        code: String,
        state: Option<String>,
    },
    Token {
        access_token: String,
        token_type: String,
        expires_in: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    IdToken {
        id_token: String,
        token_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    TokenIdToken {
        access_token: String,
        token_type: String,
        expires_in: i64,
        id_token: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    HybridToken {
        code: String,
        access_token: String,
        token_type: String,
        expires_in: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    HybridIdToken {
        code: String,
        id_token: String,
        token_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    HybridTokenIdToken {
        code: String,
        access_token: String,
        token_type: String,
        expires_in: i64,
        id_token: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    RequireAcceptance {
        grant_id: String,
        scope: Vec<Scope>,
    },
    Error {
        error: String,
        error_description: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
        #[serde(skip_serializing)]
        _cause: ed::Error,
    },
}

pub struct AuthorizeRet {
    pub redirect_uri: Option<String>,
    pub kind: AuthorizeRetKind,
}

impl AuthorizeRet {
    pub fn require_acceptance(grant_id: String, scope: Vec<Scope>) -> Self {
        AuthorizeRet {
            redirect_uri: None,
            kind: AuthorizeRetKind::RequireAcceptance { grant_id, scope },
        }
    }

    pub fn error(error: ed::Error, redirect_uri: Option<String>, state: Option<String>) -> Self {
        AuthorizeRet {
            redirect_uri,
            kind: AuthorizeRetKind::Error {
                error: error.description().to_string(),
                error_description: error.to_string(),
                state,
                _cause: error,
            },
        }
    }

    pub fn builder(redirect_uri: String, state: Option<String>) -> AuthorizeRetBuilder {
        AuthorizeRetBuilder::new(redirect_uri, state)
    }
}

pub struct AuthorizeRetBuilder {
    redirect_uri: String,
    state: Option<String>,
    code: Option<String>,
    access_token: Option<String>,
    expires_in: i64,
    id_token: Option<String>,
    kind: u32,
}

impl AuthorizeRetBuilder {
    fn new(redirect_uri: String, state: Option<String>) -> Self {
        AuthorizeRetBuilder {
            redirect_uri,
            state,
            code: None,
            access_token: None,
            expires_in: -1i64,
            id_token: None,
            kind: 0b000,
        }
    }

    pub fn code(self, code: Option<String>) -> Self {
        match code {
            Some(c) => AuthorizeRetBuilder {
                code: Some(c),
                kind: self.kind | 0b001,
                ..self
            },
            None => self,
        }
    }

    pub fn access_token(self, access_token: Option<AccessToken>) -> Self {
        match access_token {
            Some(a) => AuthorizeRetBuilder {
                access_token: Some(a.token),
                expires_in: a.expires_in,
                kind: self.kind | 0b010,
                ..self
            },
            None => self,
        }
    }

    pub fn id_token(self, id_token: Option<IdToken>) -> Self {
        match id_token {
            Some(i) => AuthorizeRetBuilder {
                id_token: Some(i.token),
                kind: self.kind | 0b100,
                ..self
            },
            None => self,
        }
    }

    pub fn build(self) -> Option<AuthorizeRet> {
        let kind = match self.kind {
            0b001 => AuthorizeRetKind::Code {
                code: self.code.unwrap(),
                state: self.state,
            },
            0b010 => AuthorizeRetKind::Token {
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                state: self.state,
            },
            0b011 => AuthorizeRetKind::HybridToken {
                code: self.code.unwrap(),
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                state: self.state,
            },
            0b100 => AuthorizeRetKind::IdToken {
                id_token: self.id_token.unwrap(),
                token_type: "Bearer".to_string(),
                state: self.state,
            },
            0b101 => AuthorizeRetKind::HybridIdToken {
                code: self.code.unwrap(),
                id_token: self.id_token.unwrap(),
                token_type: "Bearer".to_string(),
                state: self.state,
            },
            0b110 => AuthorizeRetKind::TokenIdToken {
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                id_token: self.id_token.unwrap(),
                state: self.state,
            },
            0b111 => AuthorizeRetKind::HybridTokenIdToken {
                code: self.code.unwrap(),
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                id_token: self.id_token.unwrap(),
                state: self.state,
            },
            _ => return None,
        };

        Some(AuthorizeRet {
            redirect_uri: Some(self.redirect_uri),
            kind,
        })
    }
}
