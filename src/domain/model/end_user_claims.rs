use chrono::prelude::*;

use domain::model::EndUser;

/// `EndUserClaims` is the type represents claims of an end user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_time: Option<i64>,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoneinfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EndUserClaims {
    /// Generates EndUserClaims from EndUser.
    pub fn from_end_user(issuer: &str, end_user: &EndUser, client_id: &String) -> Self {
        EndUserClaims {
            iss: issuer.to_string(),
            sub: end_user.id.clone(),
            aud: client_id.clone(),
            auth_time: end_user.authenticated_at.as_ref().map(|t| t.timestamp()),
            name: end_user.name.clone(),
            email: end_user.email.clone(),
            given_name: end_user.given_name.clone(),
            family_name: end_user.family_name.clone(),
            middle_name: end_user.middle_name.clone(),
            nickname: end_user.nickname.clone(),
            profile: end_user.profile.clone(),
            picture: end_user.picture.clone(),
            website: end_user.website.clone(),
            gender: end_user.gender.clone(),
            birthdate: end_user.birthdate.clone(),
            zoneinfo: end_user.zoneinfo.clone(),
            locale: end_user.locale.clone(),
            phone_number: end_user.phone_number.clone(),
            created_at: end_user.created_at.clone(),
            updated_at: end_user.updated_at.clone(),
        }
    }
}
