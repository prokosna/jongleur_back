use domain::error::domain as ed;
use domain::model::IdToken;

pub trait IdTokenRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<IdToken>, ed::Error>;
    fn find_by_token(&self, token: &String) -> Result<Option<IdToken>, ed::Error>;
    fn find_all(&self) -> Result<Vec<IdToken>, ed::Error>;
    fn add(&self, model: &IdToken) -> Result<(), ed::Error>;
    fn update(&self, model: &IdToken) -> Result<(), ed::Error>;
    fn remove(&self, model: IdToken) -> Result<(), ed::Error>;
}

pub trait IdTokenRepositoryComponent {
    type IdTokenRepository: IdTokenRepository;
    fn id_token_repository(&self) -> &Self::IdTokenRepository;
}
