use chrono::prelude::*;
use uuid::Uuid;

use domain::error::domain as eg;
use util::generate_uid;
use util::hash_str;
use self::eg::ResultExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    #[serde(rename = "_id")] pub id: String,
    pub name: String,
    pub password: String,
    pub website: String,
    pub resource_secret: String,
    pub scope: Vec<Scope>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_valid: bool,
}

impl Resource {
    pub fn authenticate(&self, password: &String) -> bool {
        self.password == hash_str(&password)
    }
}

pub struct ResourceBuilder {
    name: String,
    password: String,
    website: String,
    scope: Vec<Scope>,
}

impl ResourceBuilder {
    pub fn new(name: String, password: String, website: String) -> Self {
        ResourceBuilder {
            name,
            password: hash_str(&password),
            website,
            scope: Vec: new(),
        }
    }

    pub fn scope(self, scope: Vec<Scope>) -> Self {
        ResourceBuilder {
            scope: [&self.scope[..], &scope[..]].concat(),
            ..self
        }
    }

    pub fn build(self) -> self::eg::Result<Resource> {
        let created_at = Utc::now();
        Ok(Resource {
            id: Uuid::new_v4().simple().to_string(),
            name: self.name,
            password: self.password,
            website: self.website,
            scope: self.scope,
            resource_secret: generate_uid(64usize).chain_err(|| "generating uid failed")?,
            created_at,
            updated_at: created_at,
            is_valid: true,
        })
    }
}
