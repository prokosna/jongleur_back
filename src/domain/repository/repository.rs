use bson::Document;
use domain::error::domain as ed;

pub trait Repository<T> {
    fn insert(&self, model: &T) -> Result<(), ed::Error>;
    fn find(&self, query: &Document) -> Result<Vec<T>, ed::Error>;
    fn find_by_key(&self, key: &str) -> Result<Option<T>, ed::Error>;
    fn update(&self, model: &T) -> Result<(), ed::Error>;
    fn upsert(&self, model: &T) -> Result<(), ed::Error>;
    fn remove(&self, model: &T) -> Result<(), ed::Error>;
    fn remove_by_key(&self, key: &str) -> Result<(), ed::Error>;
}
