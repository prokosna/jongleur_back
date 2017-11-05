#[derive(FromForm)]
pub struct IntrospectParameters {
    pub token: String,
    pub token_type_hint: Option<String>,
}
