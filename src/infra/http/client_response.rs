use chrono::prelude::*;
use domain::model::{Client, ClientType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawClientResponse {
    id: String,
    name: String,
    website: String,
    client_type: ClientType,
    client_secret: String,
    resource_id: String,
    redirect_uris: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RawClientResponse {
    pub fn from_client(client: &Client) -> Self {
        RawClientResponse {
            id: client.id.clone(),
            name: client.name.clone(),
            website: client.website.clone(),
            client_type: client.client_type.clone(),
            client_secret: client.client_secret.clone(),
            resource_id: client.resource_id.clone(),
            redirect_uris: client.redirect_uris.clone(),
            created_at: client.created_at.clone(),
            updated_at: client.updated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientResponse {
    id: String,
    name: String,
    website: String,
    resource_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ClientResponse {
    pub fn from_client(client: &Client) -> Self {
        ClientResponse {
            id: client.id.clone(),
            name: client.name.clone(),
            website: client.website.clone(),
            resource_id: client.resource_id.clone(),
            created_at: client.created_at.clone(),
            updated_at: client.updated_at.clone(),
        }
    }
}
