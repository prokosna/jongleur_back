use chrono::prelude::*;
use std::ops::IndexMut;

use domain::error::domain as ed;
use util::{generate_random_id, hash_str};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceptedClient {
    pub client_id: String,
    pub scope: Vec<String>,
}

/// `EndUser` is an end user in the context of OAuth2 and OpenID Connect.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUser {
    pub id: String,
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
    pub authenticated_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

impl EndUser {
    /// Authenticates the end user by a password.
    pub fn is_authenticated(&self, password: &String) -> bool {
        self.password == hash_str(&password)
    }

    pub fn update_authenticated_timestamp(&mut self) {
        self.authenticated_at = Some(Utc::now());
    }

    /// Adds a new accepted client to the end user.
    pub fn add_accepted_client(&mut self, client_id: &String, scope: &Vec<String>) {
        let client = AcceptedClient {
            client_id: client_id.clone(),
            scope: scope.clone(),
        };
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

    /// Returns true if the scope contains new scope which requires new acceptance of this end user.
    pub fn require_acceptance(&self, scope: &Vec<String>, client_id: &String) -> bool {
        !(self.accepted_clients.iter().any(|c| {
            &c.client_id == client_id && !scope.iter().any(|s1| !c.scope.iter().any(|s2| s1 == s2))
        }))
    }

    pub fn update_password(
        &mut self,
        new_password: &String,
        current_password: &String,
    ) -> Result<(), ed::Error> {
        if self.is_authenticated(current_password) {
            self.password = hash_str(new_password);
            Ok(())
        } else {
            Err(ed::ErrorKind::WrongPassword(format!("{}", self.id)).into())
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn builder(name: &String, password: &String, email: &String) -> EndUserBuilder {
        EndUserBuilder {
            name: name.clone(),
            password: hash_str(&password),
            email: email.clone(),
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
    pub fn given_name(self, given_name: &Option<String>) -> Self {
        EndUserBuilder {
            given_name: given_name.clone(),
            ..self
        }
    }
    pub fn family_name(self, family_name: &Option<String>) -> Self {
        EndUserBuilder {
            family_name: family_name.clone(),
            ..self
        }
    }
    pub fn middle_name(self, middle_name: &Option<String>) -> Self {
        EndUserBuilder {
            middle_name: middle_name.clone(),
            ..self
        }
    }
    pub fn nickname(self, nickname: &Option<String>) -> Self {
        EndUserBuilder {
            nickname: nickname.clone(),
            ..self
        }
    }
    pub fn profile(self, profile: &Option<String>) -> Self {
        EndUserBuilder {
            profile: profile.clone(),
            ..self
        }
    }
    pub fn picture(self, picture: &Option<String>) -> Self {
        EndUserBuilder {
            picture: picture.clone(),
            ..self
        }
    }
    pub fn website(self, website: &Option<String>) -> Self {
        EndUserBuilder {
            website: website.clone(),
            ..self
        }
    }
    pub fn gender(self, gender: &Option<String>) -> Self {
        EndUserBuilder {
            gender: gender.clone(),
            ..self
        }
    }
    pub fn birthdate(self, birthdate: &Option<NaiveDate>) -> Self {
        EndUserBuilder {
            birthdate: birthdate.clone(),
            ..self
        }
    }
    pub fn zoneinfo(self, zoneinfo: &Option<String>) -> Self {
        EndUserBuilder {
            zoneinfo: zoneinfo.clone(),
            ..self
        }
    }
    pub fn locale(self, locale: &Option<String>) -> Self {
        EndUserBuilder {
            locale: locale.clone(),
            ..self
        }
    }
    pub fn phone_number(self, phone_number: &Option<String>) -> Self {
        EndUserBuilder {
            phone_number: phone_number.clone(),
            ..self
        }
    }
    pub fn build(self) -> EndUser {
        let created_at = Utc::now();
        EndUser {
            id: generate_random_id(32usize),
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
            authenticated_at: None,
            is_deleted: false,
        }
    }
}
