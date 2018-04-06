use domain::error::domain as ed;
use domain::model::Client;
use domain::repository::ClientRepository;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct ClientRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for Client to use repository
impl MongoModel for Client {
    fn collection_name() -> String {
        "clients".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl ClientRepository for ClientRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<Client>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_name(&self, name: &String) -> Result<Option<Client>, ed::Error> {
        let query = doc! {"name" => name, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<Client>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &Client) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &Client) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: Client) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
