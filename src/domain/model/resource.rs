use chrono::prelude::*;

use domain::error::domain as ed;
use util::{generate_random_id, hash_str};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub password: String,
    pub website: String,
    pub resource_secret: String,
    pub scope: Vec<Scope>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Resource {
    /// Authenticates this resource by the password.
    pub fn is_authenticated(&self, password: &String) -> bool {
        self.password == hash_str(&password)
    }

    /// Returns `Vec<String>` that represents scope in this `Resource`.
    pub fn filter_scope(&self, scope: &Vec<String>) -> Vec<String> {
        scope
            .iter()
            .map(|s| s.to_string())
            .filter(|s| self.is_valid_scope(s))
            .collect()
    }

    /// Returns `Vec<Scope>` that represents scope with description.
    pub fn convert_scope(&self, scope: &Vec<String>) -> Vec<Scope> {
        self.scope
            .iter()
            .filter(|x| scope.iter().any(|s| s == &x.name))
            .map(|x| x.clone())
            .collect()
    }

    /// Returns true if the scope is valid one.
    pub fn is_valid_scope(&self, scope: &String) -> bool {
        self.scope.iter().any(|s| scope == &s.name)
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

    pub fn builder(name: &String, password: &String, website: &String) -> ResourceBuilder {
        ResourceBuilder {
            name: name.clone(),
            password: password.clone(),
            website: website.clone(),
            scope: Vec::new(),
        }
    }
}

pub struct ResourceBuilder {
    name: String,
    password: String,
    website: String,
    scope: Vec<Scope>,
}

impl ResourceBuilder {
    pub fn scope(self, scope: &Vec<Scope>) -> Self {
        ResourceBuilder {
            scope: [&self.scope[..], &scope[..]].concat(),
            ..self
        }
    }

    pub fn build(self) -> Resource {
        let created_at = Utc::now();
        Resource {
            id: generate_random_id(32usize),
            name: self.name,
            password: hash_str(&self.password),
            website: self.website,
            scope: self.scope,
            resource_secret: generate_random_id(32usize),
            created_at,
            updated_at: created_at,
            is_deleted: false,
        }
    }
}
