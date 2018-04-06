use domain::error::domain as ed;
use domain::model::Resource;
use domain::repository::ResourceRepository;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct ResourceRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for Ping to use repository
impl MongoModel for Resource {
    fn collection_name() -> String {
        "resources".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl ResourceRepository for ResourceRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<Resource>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_name(&self, name: &String) -> Result<Option<Resource>, ed::Error> {
        let query = doc! {"name" => name, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<Resource>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &Resource) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &Resource) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: Resource) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
