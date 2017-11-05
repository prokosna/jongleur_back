#[derive(FromForm)]
pub struct TokensParameters {
    pub grant_type: String,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}
