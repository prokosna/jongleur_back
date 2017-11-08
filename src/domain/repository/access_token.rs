use domain::error::domain as ed;
use domain::model::AccessToken;

pub trait AccessTokenRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<AccessToken>, ed::Error>;
    fn find_by_token(&self, token: &String) -> Result<Option<AccessToken>, ed::Error>;
    fn find_all(&self) -> Result<Vec<AccessToken>, ed::Error>;
    fn add(&self, model: &AccessToken) -> Result<(), ed::Error>;
    fn update(&self, model: &AccessToken) -> Result<(), ed::Error>;
    fn remove(&self, model: AccessToken) -> Result<(), ed::Error>;
}

pub trait AccessTokenRepositoryComponent {
    type AccessTokenRepository: AccessTokenRepository;
    fn access_token_repository(&self) -> &Self::AccessTokenRepository;
}
