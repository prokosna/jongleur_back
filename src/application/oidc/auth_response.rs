use domain::model::Scope;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthResponseKind {
    Code { code: String, state: Option<String> },
    Token {
        access_token: String,
        token_type: String,
        expires_in: i64,
        #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
    },
    IdToken {
        id_token: String,
        token_type: String,
        #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
    },
    TokenIdToken {
        access_token: String,
        token_type: String,
        expires_in: i64,
        id_token: String,
        #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
    },
    HybridToken {
        code: String,
        access_token: String,
        token_type: String,
        expires_in: i64,
        #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
    },
    HybridIdToken {
        code: String,
        id_token: String,
        token_type: String,
        #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
    },
    HybridTokenIdToken {
        code: String,
        access_token: String,
        token_type: String,
        expires_in: i64,
        id_token: String,
        #[serde(skip_serializing_if = "Option::is_none")] state: Option<String>,
    },
    RequireAcceptance { grant_id: String, scope: Vec<Scope> },
}

pub struct AuthResponse {
    pub redirect_uri: Option<String>,
    pub kind: AuthResponseKind,
}

impl AuthResponse {
    pub fn require_acceptance(grant_id: &String, scope: &Vec<Scope>) -> Self {
        AuthResponse {
            redirect_uri: None,
            kind: AuthResponseKind::RequireAcceptance {
                grant_id: grant_id.clone(),
                scope: scope.clone(),
            },
        }
    }
}

pub struct AuthResponseBuilder {
    redirect_uri: String,
    state: Option<String>,
    code: Option<String>,
    access_token: Option<String>,
    expires_in: i64,
    id_token: Option<String>,
    kind: u32,
}

impl AuthResponseBuilder {
    pub fn new(redirect_uri: &String, state: &Option<String>) -> Self {
        AuthResponseBuilder {
            redirect_uri: redirect_uri.clone(),
            state: state.clone(),
            code: None,
            access_token: None,
            expires_in: -1i64,
            id_token: None,
            kind: 0b000,
        }
    }

    pub fn code(self, code: &String) -> Self {
        AuthResponseBuilder {
            code: Some(code.clone()),
            kind: self.kind | 0b001,
            ..self
        }
    }

    pub fn access_token(self, access_token: &String, expires_in: i64) -> Self {
        AuthResponseBuilder {
            access_token: Some(access_token.clone()),
            expires_in,
            kind: self.kind | 0b010,
            ..self
        }
    }

    pub fn id_token(self, id_token: &String) -> Self {
        AuthResponseBuilder {
            id_token: Some(id_token.clone()),
            kind: self.kind | 0b100,
            ..self
        }
    }

    pub fn build(self) -> Option<AuthResponse> {
        let kind = match self.kind {
            0b001 => AuthResponseKind::Code {
                code: self.code.unwrap(),
                state: self.state,
            },
            0b010 => AuthResponseKind::Token {
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                state: self.state,
            },
            0b011 => AuthResponseKind::HybridToken {
                code: self.code.unwrap(),
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                state: self.state,
            },
            0b100 => AuthResponseKind::IdToken {
                id_token: self.id_token.unwrap(),
                token_type: "Bearer".to_string(),
                state: self.state,
            },
            0b101 => AuthResponseKind::HybridIdToken {
                code: self.code.unwrap(),
                id_token: self.id_token.unwrap(),
                token_type: "Bearer".to_string(),
                state: self.state,
            },
            0b110 => AuthResponseKind::TokenIdToken {
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                id_token: self.id_token.unwrap(),
                state: self.state,
            },
            0b111 => AuthResponseKind::HybridTokenIdToken {
                code: self.code.unwrap(),
                access_token: self.access_token.unwrap(),
                token_type: "Bearer".to_string(),
                expires_in: self.expires_in,
                id_token: self.id_token.unwrap(),
                state: self.state,
            },
            _ => return None,
        };

        Some(AuthResponse {
            redirect_uri: Some(self.redirect_uri),
            kind,
        })
    }
}
