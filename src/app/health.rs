use domain::error::domain as ed;
use domain::repository::{HealthRepository, HealthRepositoryComponent};
use util::generate_random_id;

pub trait HealthService: HealthRepositoryComponent {
    fn check(&self) -> Result<(), ed::Error> {
        let repository = self.health_repository();
        let id = generate_random_id(32usize);
        repository.add(&id, &"ok".to_string())?;
        let health = repository.find_by_id(&id)?;
        match health {
            Some(h) => repository.remove(&h),
            None => Err(ed::ErrorKind::ServerError("Health check failed.".to_string()).into()),
        }
    }
}

pub trait HealthServiceComponent {
    type HealthService: HealthService;
    fn health_service(&self) -> &Self::HealthService;
}

impl<T: HealthRepositoryComponent> HealthService for T {}
