use chrono::prelude::*;

use domain::error::domain as ed;
use util::{generate_random_id, hash_str};

/// `ClientType` is the type of client in the context of
/// OAuth2 and OpenID Connect
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientType {
    #[serde(rename = "confidential")]
    Confidential,
    #[serde(rename = "public")]
    Public,
}

impl ClientType {
    pub fn new(client_type: &str) -> Result<Self, ed::Error> {
        match client_type {
            "confidential" => Ok(ClientType::Confidential),
            "public" => Ok(ClientType::Public),
            _ => Err(ed::ErrorKind::InvalidRequest(format!(
                "Unsupported client type: {}",
                client_type
            )).into()),
        }
    }
}

/// `Client` is the type represents the client which
/// is used by end users to access resource.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub password: String,
    pub website: String,
    pub client_type: ClientType,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub resource_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Client {
    /// Authenticates a client by a password.
    pub fn is_authenticated_by_password(&self, password: &String) -> bool {
        self.password == hash_str(&password)
    }

    /// Authenticates a client by a client secret.
    pub fn is_authenticated_by_secret(&self, secret: &String) -> bool {
        &self.client_secret == secret
    }

    /// Returns true if the redirect_uri matches any uris of this client.
    pub fn validate_redirect_uri(&self, redirect_uri: &String) -> bool {
        self.redirect_uris.iter().any(|uri| uri == redirect_uri)
    }

    pub fn update_password(
        &mut self,
        new_password: &String,
        current_password: &String,
    ) -> Result<(), ed::Error> {
        if self.is_authenticated_by_password(current_password) {
            self.password = hash_str(new_password);
            Ok(())
        } else {
            Err(ed::ErrorKind::WrongPassword(format!("{}", self.id)).into())
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn builder(
        name: &String,
        password: &String,
        website: &String,
        resource_id: &String,
    ) -> ClientBuilder {
        ClientBuilder {
            name: name.clone(),
            password: hash_str(&password),
            website: website.clone(),
            client_type: ClientType::Confidential,
            redirect_uris: Vec::new(),
            resource_id: resource_id.clone(),
        }
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
    pub fn client_type(self, client_type: &ClientType) -> Self {
        ClientBuilder {
            client_type: client_type.clone(),
            ..self
        }
    }

    pub fn redirect_uris(self, redirect_uris: &Vec<String>) -> Self {
        ClientBuilder {
            redirect_uris: redirect_uris.clone(),
            ..self
        }
    }

    pub fn build(self) -> Client {
        let created_at = Utc::now();
        Client {
            id: generate_random_id(32usize),
            name: self.name,
            password: self.password,
            website: self.website,
            client_type: self.client_type,
            client_secret: generate_random_id(32usize),
            redirect_uris: self.redirect_uris,
            resource_id: self.resource_id,
            created_at,
            updated_at: created_at,
            is_deleted: false,
        }
    }
}
