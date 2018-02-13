use domain::error::domain as ed;
use domain::model::Admin;

pub trait AdminRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<Admin>, ed::Error>;
    fn find_by_name(&self, name: &String) -> Result<Option<Admin>, ed::Error>;
    fn find_all(&self) -> Result<Vec<Admin>, ed::Error>;
    fn add(&self, model: &Admin) -> Result<(), ed::Error>;
    fn update(&self, model: &Admin) -> Result<(), ed::Error>;
    fn remove(&self, model: Admin) -> Result<(), ed::Error>;
}

pub trait AdminRepositoryComponent {
    type AdminRepository: AdminRepository;
    fn admin_repository(&self) -> &Self::AdminRepository;
}
