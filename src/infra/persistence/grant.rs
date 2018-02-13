use serde_json;

use domain::model::{Grant, GrantStatus};
use domain::repository::GrantRepository;
use domain::error::domain as ed;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct GrantRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for Grant to use repository
impl MongoModel for Grant {
    fn collection_name() -> String {
        "grants".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl GrantRepository for GrantRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<Grant>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_code(&self, code: &String) -> Result<Option<Grant>, ed::Error> {
        let query = doc! {"code" => code, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_id_and_change_status(
        &self,
        id: &String,
        status: &GrantStatus,
    ) -> Result<Option<Grant>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        // TODO: Is this an ideal method to simply serialize a enum value?
        let temp = serde_json::to_string(status).unwrap();
        let status = temp.trim_matches('"');
        let modify = doc! {"$set": {"status": status}};
        self.mongo_client.find_and_modify(&query, &modify)
    }
    fn find_by_code_and_change_status(
        &self,
        code: &String,
        status: &GrantStatus,
    ) -> Result<Option<Grant>, ed::Error> {
        let query = doc! {"code" => code, "is_deleted": false};
        // TODO: Is this an ideal method to simply serialize a enum value?
        let temp = serde_json::to_string(status).unwrap();
        let status = temp.trim_matches('"');
        let modify = doc! {"$set": {"status": status}};
        self.mongo_client.find_and_modify(&query, &modify)
    }
    fn find_all(&self) -> Result<Vec<Grant>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &Grant) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &Grant) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: Grant) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
