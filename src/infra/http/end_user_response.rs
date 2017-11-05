use chrono::prelude::*;
use domain::model::{AcceptedClient, EndUser};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawEndUserResponse {
    id: String,
    name: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")] given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] birthdate: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")] zoneinfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] phone_number: Option<String>,
    accepted_clients: Vec<AcceptedClient>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")] last_authenticated_at: Option<DateTime<Utc>>,
}

impl RawEndUserResponse {
    pub fn from_end_user(end_user: &EndUser) -> Self {
        RawEndUserResponse {
            id: end_user.id.clone(),
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
            accepted_clients: end_user.accepted_clients.clone(),
            created_at: end_user.created_at.clone(),
            updated_at: end_user.updated_at.clone(),
            last_authenticated_at: end_user.last_authenticated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserResponse {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl EndUserResponse {
    pub fn from_end_user(end_user: &EndUser) -> Self {
        EndUserResponse {
            id: end_user.id.clone(),
            name: end_user.name.clone(),
            created_at: end_user.created_at.clone(),
            updated_at: end_user.updated_at.clone(),
        }
    }
}
