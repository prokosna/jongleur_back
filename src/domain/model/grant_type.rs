pub enum GrantType {
    RefreshToken,
    ClientCredentials,
    Password,
    AuthorizationCode,
    Undefined(String),
}

impl GrantType {
    pub fn new(grant_type: &str) -> Self {
        match grant_type.as_ref() {
            "refresh_token" => GrantType::RefreshToken,
            "client_credentials" => GrantType::ClientCredentials,
            "password" => GrantType::Password,
            "authorization_code" => GrantType::AuthorizationCode,
            _ => GrantType::Undefined(grant_type.to_string()),
        }
    }
}
