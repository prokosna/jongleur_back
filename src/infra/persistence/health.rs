use domain::error::domain as ed;
use domain::repository::{Health, HealthRepository};
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct HealthRepositoryMongo {
    pub mongo_client: MongoClient,
}

impl MongoModel for Health {
    fn collection_name() -> String {
        "health".to_string()
    }
    fn key_name() -> String {
        "_id".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl HealthRepository for HealthRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<Health>, ed::Error> {
        let query = doc! {"_id": id};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }

    fn add(&self, id: &String, text: &String) -> Result<(), ed::Error> {
        let health = Health {
            id: id.to_string(),
            text: text.to_string(),
        };
        self.mongo_client.insert(&health)
    }

    fn remove(&self, model: &Health) -> Result<(), ed::Error> {
        self.mongo_client.remove(model)
    }
}
