use chrono::prelude::*;

use domain::error::domain as ed;
use util::{generate_random_id, hash_str};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Admin {
    pub id: String,
    pub name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Admin {
    pub fn is_authenticated(&self, password: &String) -> bool {
        self.password == hash_str(password)
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

    pub fn builder(name: &String, password: &String) -> AdminBuilder {
        AdminBuilder {
            name: name.clone(),
            password: password.clone(),
        }
    }
}

pub struct AdminBuilder {
    pub name: String,
    pub password: String,
}

impl AdminBuilder {
    pub fn build(self) -> Admin {
        let created_at = Utc::now();
        Admin {
            id: generate_random_id(32usize),
            name: self.name,
            password: hash_str(&self.password),
            created_at,
            updated_at: created_at,
            is_deleted: false,
        }
    }
}
