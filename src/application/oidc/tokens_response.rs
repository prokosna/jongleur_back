#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokensResponseKind {
    Token {
        access_token: String,
        #[serde(skip_serializing_if = "Option::is_none")] refresh_token: Option<String>,
        token_type: String,
        expires_in: i64,
        #[serde(skip_serializing_if = "Option::is_none")] id_token: Option<String>,
    },
}

pub struct TokensResponse {
    pub kind: TokensResponseKind,
}

pub struct TokensResponseBuilder {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
    id_token: Option<String>,
}

impl TokensResponseBuilder {
    pub fn new(access_token: &String, expires_in: i64) -> Self {
        TokensResponseBuilder {
            access_token: access_token.clone(),
            expires_in,
            refresh_token: None,
            id_token: None,
        }
    }

    pub fn refresh_token(self, refresh_token: &String) -> Self {
        TokensResponseBuilder {
            refresh_token: Some(refresh_token.clone()),
            ..self
        }
    }

    pub fn id_token(self, id_token: &String) -> Self {
        TokensResponseBuilder {
            id_token: Some(id_token.clone()),
            ..self
        }
    }

    pub fn build(self) -> TokensResponse {
        let kind = TokensResponseKind::Token {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.expires_in,
            id_token: self.id_token,
        };

        TokensResponse { kind }
    }
}
