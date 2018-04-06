use domain::error::domain as ed;
use domain::model::EndUser;
use domain::repository::EndUserRepository;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct EndUserRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for EndUser to use repository
impl MongoModel for EndUser {
    fn collection_name() -> String {
        "end_users".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl EndUserRepository for EndUserRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<EndUser>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_name(&self, name: &String) -> Result<Option<EndUser>, ed::Error> {
        let query = doc! {"name" => name, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<EndUser>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &EndUser) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &EndUser) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: EndUser) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
