use domain::error::domain as ed;
use domain::model::Client;

pub trait ClientRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<Client>, ed::Error>;
    fn find_by_name(&self, name: &String) -> Result<Option<Client>, ed::Error>;
    fn find_by_resource_id(&self, id: &String) -> Result<Vec<Client>, ed::Error>;
    fn find_all(&self) -> Result<Vec<Client>, ed::Error>;
    fn add(&self, model: &Client) -> Result<(), ed::Error>;
    fn update(&self, model: &Client) -> Result<(), ed::Error>;
    fn remove(&self, model: Client) -> Result<(), ed::Error>;
}

pub trait ClientRepositoryComponent {
    type ClientRepository: ClientRepository;
    fn client_repository(&self) -> &Self::ClientRepository;
}
