use domain::error::domain as ed;
use domain::model::RefreshToken;

pub trait RefreshTokenRepository {
    fn find_by_token(&self, token: &String) -> Result<Option<RefreshToken>, ed::Error>;
    fn find_all(&self) -> Result<Vec<RefreshToken>, ed::Error>;
    fn add(&self, model: &RefreshToken) -> Result<(), ed::Error>;
    fn update(&self, model: &RefreshToken) -> Result<(), ed::Error>;
    fn remove(&self, model: RefreshToken) -> Result<(), ed::Error>;
}

pub trait RefreshTokenRepositoryComponent {
    type RefreshTokenRepository: RefreshTokenRepository;
    fn refresh_token_repository(&self) -> &Self::RefreshTokenRepository;
}
