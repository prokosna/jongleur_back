use chrono::prelude::*;
use domain::model::{Resource, Scope};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawResourceResponse {
    id: String,
    name: String,
    website: String,
    scope: Vec<Scope>,
    resource_secret: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RawResourceResponse {
    pub fn from_resource(resource: &Resource) -> Self {
        RawResourceResponse {
            id: resource.id.clone(),
            name: resource.name.clone(),
            website: resource.website.clone(),
            resource_secret: resource.resource_secret.clone(),
            scope: resource.scope.clone(),
            created_at: resource.created_at.clone(),
            updated_at: resource.updated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceResponse {
    id: String,
    name: String,
    website: String,
    scope: Vec<Scope>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ResourceResponse {
    pub fn from_resource(resource: &Resource) -> Self {
        ResourceResponse {
            id: resource.id.clone(),
            name: resource.name.clone(),
            website: resource.website.clone(),
            scope: resource.scope.clone(),
            created_at: resource.created_at.clone(),
            updated_at: resource.updated_at.clone(),
        }
    }
}
