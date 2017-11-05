use chrono::prelude::*;
use uuid::Uuid;

use util::generate_uid;
use util::hash_str;
use domain::error::general as eg;
use self::eg::ResultExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientType {
    #[serde(rename = "Confidential")] Confidential,
    #[serde(rename = "Public")] Public,
}

impl ClientType {
    pub fn new(client_type: &str) -> Result<Self, eg::Error> {
        match client_type {
            "Confidential" => Ok(ClientType::Confidential),
            "Public" => Ok(ClientType::Public),
            _ => Err("Unsupported client type".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    #[serde(rename = "_id")] pub id: String,
    pub name: String,
    pub password: String,
    pub website: String,
    pub client_type: ClientType,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub resource_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_valid: bool,
}

impl Client {
    pub fn authenticate_by_password(&self, password: &String) -> bool {
        self.password == hash_str(&password)
    }

    pub fn authenticate_by_secret(&self, secret: &String) -> bool {
        &self.client_secret == secret
    }
}

pub struct ClientBuilder {
    name: String,
    password: String,
    website: String,
    client_type: ClientType,
    redirect_uris: Vec<String>,
    resource_id: String,
}

impl ClientBuilder {
    pub fn new(name: String, password: String, website: String, resource_id: String) -> Self {
        ClientBuilder {
            name,
            password: hash_str(&password),
            website,
            client_type: ClientType::Confidential,
            redirect_uris: Vec::new(),
            resource_id,
        }
    }

    pub fn client_type(self, client_type: ClientType) -> Self {
        ClientBuilder {
            client_type,
            ..self
        }
    }

    pub fn redirect_uris(self, redirect_uris: Vec<String>) -> Self {
        ClientBuilder {
            redirect_uris,
            ..self
        }
    }

    pub fn build(self) -> self::eg::Result<Client> {
        let created_at = Utc::now();
        Ok(Client {
            id: Uuid::new_v4().simple().to_string(),
            name: self.name,
            password: self.password,
            website: self.website,
            client_type: self.client_type,
            client_secret: generate_uid(64usize).chain_err(|| "generating uid failed")?,
            redirect_uris: self.redirect_uris,
            resource_id: self.resource_id,
            created_at,
            updated_at: created_at,
            is_valid: true,
        })
    }
}
