use config::AppConfig;

#[derive(Serialize, Deserialize)]
pub struct IntrospectRet {
    action: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nbf: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aud: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jti: Option<String>,
}

impl IntrospectRet {
    pub fn builder(action: bool) -> IntrospectRetBuilder {
        IntrospectRetBuilder::new(action)
    }
}

pub struct IntrospectRetBuilder {
    action: bool,
    scope: Option<String>,
    client_id: Option<String>,
    username: Option<String>,
    token_type: Option<String>,
    exp: Option<i64>,
    iat: Option<i64>,
    nbf: Option<i64>,
    sub: Option<String>,
    aud: Option<String>,
    jti: Option<String>,
}

impl IntrospectRetBuilder {
    fn new(action: bool) -> Self {
        IntrospectRetBuilder {
            action,
            scope: None,
            client_id: None,
            username: None,
            token_type: Some("Bearer".to_string()),
            exp: None,
            iat: None,
            nbf: None,
            sub: None,
            aud: None,
            jti: None,
        }
    }

    pub fn scope(self, scope: Option<String>) -> Self {
        IntrospectRetBuilder { scope, ..self }
    }
    pub fn client_id(self, client_id: Option<String>) -> Self {
        IntrospectRetBuilder { client_id, ..self }
    }
    pub fn username(self, username: Option<String>) -> Self {
        IntrospectRetBuilder { username, ..self }
    }
    pub fn token_type(self, token_type: Option<String>) -> Self {
        IntrospectRetBuilder { token_type, ..self }
    }
    pub fn exp(self, exp: Option<i64>) -> Self {
        IntrospectRetBuilder { exp, ..self }
    }
    pub fn iat(self, iat: Option<i64>) -> Self {
        IntrospectRetBuilder { iat, ..self }
    }
    pub fn nbf(self, nbf: Option<i64>) -> Self {
        IntrospectRetBuilder { nbf, ..self }
    }
    pub fn sub(self, sub: Option<String>) -> Self {
        IntrospectRetBuilder { sub, ..self }
    }
    pub fn aud(self, aud: Option<String>) -> Self {
        IntrospectRetBuilder { aud, ..self }
    }
    pub fn jti(self, jti: Option<String>) -> Self {
        IntrospectRetBuilder { jti, ..self }
    }
    pub fn build(self) -> IntrospectRet {
        IntrospectRet {
            action: self.action,
            scope: self.scope,
            client_id: self.client_id,
            username: self.username,
            token_type: self.token_type,
            exp: self.exp,
            iat: self.iat,
            nbf: self.nbf,
            sub: self.sub,
            aud: self.aud,
            iss: Some(AppConfig::issuer().to_string()),
            jti: self.jti,
        }
    }
}
