use domain::error::domain as ed;
use domain::model::{Grant, GrantStatus};

pub trait GrantRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<Grant>, ed::Error>;
    fn find_by_code(&self, code: &String) -> Result<Option<Grant>, ed::Error>;
    fn find_by_id_and_change_status(
        &self,
        id: &String,
        status: &GrantStatus,
    ) -> Result<Option<Grant>, ed::Error>;
    fn find_by_code_and_change_status(
        &self,
        code: &String,
        status: &GrantStatus,
    ) -> Result<Option<Grant>, ed::Error>;
    fn find_all(&self) -> Result<Vec<Grant>, ed::Error>;
    fn add(&self, model: &Grant) -> Result<(), ed::Error>;
    fn update(&self, model: &Grant) -> Result<(), ed::Error>;
    fn remove(&self, model: Grant) -> Result<(), ed::Error>;
}

pub trait GrantRepositoryComponent {
    type GrantRepository: GrantRepository;
    fn grant_repository(&self) -> &Self::GrantRepository;
}
