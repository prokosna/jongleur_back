use domain::model::IdToken;
use domain::repository::IdTokenRepository;
use domain::error::domain as ed;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct IdTokenRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for IdToken to use repository
impl MongoModel for IdToken {
    fn collection_name() -> String {
        "id_tokens".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl IdTokenRepository for IdTokenRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<IdToken>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_token(&self, token: &String) -> Result<Option<IdToken>, ed::Error> {
        let query = doc! {"token" => token, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<IdToken>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &IdToken) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &IdToken) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: IdToken) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
