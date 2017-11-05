use chrono::prelude::*;
use uuid::Uuid;
use std::ops::IndexMut;

use util::hash_str;
use domain::error::general as eg;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceptedClient {
    pub client_id: String,
    pub scope: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUser {
    #[serde(rename = "_id")] pub id: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_verified: bool,
    pub accepted_clients: Vec<AcceptedClient>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_authenticated_at: Option<DateTime<Utc>>,
    pub is_valid: bool,
}

impl EndUser {
    pub fn authenticate(&mut self, password: &String) -> bool {
        if self.password == hash_str(&password) {
            self.last_authenticated_at = Some(Utc::now());
            true
        } else {
            false
        }
    }

    pub fn add_accepted_client(&mut self, client: AcceptedClient) -> () {
        let pos = self.accepted_clients
            .iter()
            .position(|c| c.client_id == client.client_id);
        match pos {
            Some(i) => {
                let c = self.accepted_clients.index_mut(i);
                let new: Vec<String> = client
                    .scope
                    .iter()
                    .filter(|s1| !c.scope.iter().any(|s2| s1 == &s2))
                    .map(|s| s.to_string())
                    .collect();
                let concat = [&c.scope[..], &new[..]].concat();
                c.scope = concat;
            }
            None => {
                self.accepted_clients.push(client);
            }
        }
    }
}

pub struct EndUserBuilder {
    name: String,
    password: String,
    email: String,
    given_name: Option<String>,
    family_name: Option<String>,
    middle_name: Option<String>,
    nickname: Option<String>,
    profile: Option<String>,
    picture: Option<String>,
    website: Option<String>,
    gender: Option<String>,
    birthdate: Option<NaiveDate>,
    zoneinfo: Option<String>,
    locale: Option<String>,
    phone_number: Option<String>,
}

impl EndUserBuilder {
    pub fn new(name: String, password: String, email: String) -> Self {
        EndUserBuilder {
            name,
            password: hash_str(&password),
            email,
            given_name: None,
            family_name: None,
            middle_name: None,
            nickname: None,
            profile: None,
            picture: None,
            website: None,
            gender: None,
            birthdate: None,
            zoneinfo: None,
            locale: None,
            phone_number: None,
        }
    }

    pub fn given_name(self, given_name: Option<String>) -> Self {
        EndUserBuilder { given_name, ..self }
    }
    pub fn family_name(self, family_name: Option<String>) -> Self {
        EndUserBuilder {
            family_name,
            ..self
        }
    }
    pub fn middle_name(self, middle_name: Option<String>) -> Self {
        EndUserBuilder {
            middle_name,
            ..self
        }
    }
    pub fn nickname(self, nickname: Option<String>) -> Self {
        EndUserBuilder { nickname, ..self }
    }
    pub fn profile(self, profile: Option<String>) -> Self {
        EndUserBuilder { profile, ..self }
    }
    pub fn picture(self, picture: Option<String>) -> Self {
        EndUserBuilder { picture, ..self }
    }
    pub fn website(self, website: Option<String>) -> Self {
        EndUserBuilder { website, ..self }
    }
    pub fn gender(self, gender: Option<String>) -> Self {
        EndUserBuilder { gender, ..self }
    }
    pub fn birthdate(self, birthdate: Option<NaiveDate>) -> Self {
        EndUserBuilder { birthdate, ..self }
    }
    pub fn zoneinfo(self, zoneinfo: Option<String>) -> Self {
        EndUserBuilder { zoneinfo, ..self }
    }
    pub fn locale(self, locale: Option<String>) -> Self {
        EndUserBuilder { locale, ..self }
    }
    pub fn phone_number(self, phone_number: Option<String>) -> Self {
        EndUserBuilder {
            phone_number,
            ..self
        }
    }
    pub fn build(self) -> eg::Result<EndUser> {
        let created_at = Utc::now();
        Ok(EndUser {
            id: Uuid::new_v4().simple().to_string(),
            password: self.password,
            name: self.name,
            email: self.email,
            email_verified: false,
            given_name: self.given_name,
            family_name: self.family_name,
            middle_name: self.middle_name,
            nickname: self.nickname,
            profile: self.profile,
            picture: self.picture,
            website: self.website,
            gender: self.gender,
            birthdate: self.birthdate,
            zoneinfo: self.zoneinfo,
            locale: self.locale,
            phone_number: self.phone_number,
            phone_number_verified: false,
            accepted_clients: Vec::new(),
            created_at,
            updated_at: created_at,
            last_authenticated_at: None,
            is_valid: true,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub auth_time: Option<i64>,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub birthdate: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")] pub zoneinfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EndUserClaims {
    pub fn from_end_user(issuer: &str, end_user: &EndUser, client_id: &String) -> Self {
        EndUserClaims {
            iss: issuer.to_string(),
            sub: end_user.id.clone(),
            aud: client_id.clone(),
            auth_time: end_user
                .last_authenticated_at
                .as_ref()
                .map(|t| t.timestamp()),
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
