use domain::error::domain as ed;
use domain::model::Resource;

pub trait ResourceRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<Resource>, ed::Error>;
    fn find_by_name(&self, name: &String) -> Result<Option<Resource>, ed::Error>;
    fn find_all(&self) -> Result<Vec<Resource>, ed::Error>;
    fn add(&self, model: &Resource) -> Result<(), ed::Error>;
    fn update(&self, model: &Resource) -> Result<(), ed::Error>;
    fn remove(&self, model: Resource) -> Result<(), ed::Error>;
}

pub trait ResourceRepositoryComponent {
    type ResourceRepository: ResourceRepository;
    fn resource_repository(&self) -> &Self::ResourceRepository;
}
