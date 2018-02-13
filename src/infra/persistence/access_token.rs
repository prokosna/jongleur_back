use domain::model::AccessToken;
use domain::repository::AccessTokenRepository;
use domain::error::domain as ed;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct AccessTokenRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for AccessToken to use repository
impl MongoModel for AccessToken {
    fn collection_name() -> String {
        "access_tokens".to_string()
    }
    fn key_value(&self) -> String {
        self.id.clone()
    }
}

impl AccessTokenRepository for AccessTokenRepositoryMongo {
    fn find_by_id(&self, id: &String) -> Result<Option<AccessToken>, ed::Error> {
        let query = doc! {"id" => id, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_by_token(&self, token: &String) -> Result<Option<AccessToken>, ed::Error> {
        let query = doc! {"token" => token, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<AccessToken>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &AccessToken) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &AccessToken) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: AccessToken) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
