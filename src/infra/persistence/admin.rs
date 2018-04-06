use domain::error::domain as ed;
use domain::model::Admin;
use domain::repository::AdminRepository;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct AdminRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for Admin to use repository
impl MongoModel for Admin {
    fn collection_name() -> String {
        "admins".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl AdminRepository for AdminRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<Admin>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_name(&self, name: &String) -> Result<Option<Admin>, ed::Error> {
        let query = doc! {"name" => name, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<Admin>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &Admin) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &Admin) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: Admin) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
