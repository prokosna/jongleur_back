use domain::error::domain as ed;

pub trait Store {
    fn set(&self, key: &str, field: &str, value: &str) -> Result<(), ed::Error>;
    fn get(&self, key: &str, field: &str) -> Result<String, ed::Error>;
    fn del(&self, key: &str, field: Option<&str>) -> Result<(), ed::Error>;
}
