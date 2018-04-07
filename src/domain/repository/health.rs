use domain::error::domain as ed;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    #[serde(rename = "_id")]
    pub id: String,
    pub text: String,
}

pub trait HealthRepository {
    fn find_by_id(&self, id: &String) -> Result<Option<Health>, ed::Error>;
    fn add(&self, id: &String, text: &String) -> Result<(), ed::Error>;
    fn remove(&self, id: &Health) -> Result<(), ed::Error>;
}

pub trait HealthRepositoryComponent {
    type HealthRepository: HealthRepository;
    fn health_repository(&self) -> &Self::HealthRepository;
}
