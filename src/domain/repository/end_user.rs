use domain::error::domain as ed;
use domain::model::EndUser;

pub trait EndUserRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<EndUser>, ed::Error>;
    fn find_by_name(&self, name: &String) -> Result<Option<EndUser>, ed::Error>;
    fn find_all(&self) -> Result<Vec<EndUser>, ed::Error>;
    fn add(&self, model: &EndUser) -> Result<(), ed::Error>;
    fn update(&self, model: &EndUser) -> Result<(), ed::Error>;
    fn remove(&self, model: EndUser) -> Result<(), ed::Error>;
}

pub trait EndUserRepositoryComponent {
    type EndUserRepository: EndUserRepository;
    fn end_user_repository(&self) -> &Self::EndUserRepository;
}
