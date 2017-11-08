use domain::model::RefreshToken;
use domain::repository::RefreshTokenRepository;
use domain::error::domain as ed;
use infra::persistence::{MongoClient, MongoModel};

#[derive(Clone)]
pub struct RefreshTokenRepositoryMongo {
    pub mongo_client: MongoClient,
}

// Implement MongoModel for RefreshToken to use repository
impl MongoModel for RefreshToken {
    fn collection_name() -> String {
        "refresh_tokens".to_string()
    }
    fn key_value(&self) -> String {
        self.token.clone()
    }
}

impl RefreshTokenRepository for RefreshTokenRepositoryMongo {
    fn find_by_token(&self, token: &String) -> Result<Option<RefreshToken>, ed::Error> {
        let query = doc! {"token" => token, "is_deleted": false};
        self.mongo_client.find(&query).map(|mut v| v.pop())
    }
    fn find_all(&self) -> Result<Vec<RefreshToken>, ed::Error> {
        let query = doc! {"is_deleted": false};
        self.mongo_client.find(&query)
    }
    fn add(&self, model: &RefreshToken) -> Result<(), ed::Error> {
        self.mongo_client.insert(model)
    }
    fn update(&self, model: &RefreshToken) -> Result<(), ed::Error> {
        self.mongo_client.update(model)
    }
    fn remove(&self, mut model: RefreshToken) -> Result<(), ed::Error> {
        model.is_deleted = true;
        self.mongo_client.update(&model)
    }
}
