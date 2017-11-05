use bson::Document;
use mongo_driver::{MongoError, MongoErrorCode};

use domain::repository::Repository;
use infra::db::MongoModel;
use infra::db::MongoClient;
use domain::error::domain as ed;
use self::ed::ResultExt;

pub struct MongoRepository<'a> {
    client: &'a MongoClient<'a>,
}

impl<'a> MongoRepository<'a> {
    pub fn new(mongo_client: &'a MongoClient) -> Self {
        MongoRepository {
            client: mongo_client,
        }
    }
}

impl<'a, T> Repository<T> for MongoRepository<'a>
where
    T: MongoModel,
{
    fn insert(&self, model: &T) -> Result<(), ed::Error> {
        let ret = self.client.insert(&T::collection_name(), &model.to_doc());
        match ret {
            Ok(_) => Ok(()),
            Err(e) => match e {
                MongoError::Bsonc(e) => match e.code() {
                    MongoErrorCode::DuplicateKey => {
                        Err(ed::Error::with_chain(e, ed::ErrorKind::DuplicatedEntry))
                    }
                    _ => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ServerError(
                            "Unexpected error occurred when inserting the document".to_string(),
                        ),
                    )),
                },
                _ => Err(ed::Error::with_chain(
                    e,
                    ed::ErrorKind::ServerError(
                        "Unexpected error occurred when inserting the document".to_string(),
                    ),
                )),
            },
        }
    }

    fn find(&self, query: &Document) -> Result<Vec<T>, ed::Error> {
        self.client
            .find(&T::collection_name(), &query)
            .map(|ret| ret.iter().map(|doc| T::from_doc(&doc)).collect())
            .chain_err(|| {
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when finding the document".to_string(),
                )
            })
    }

    fn find_by_key(&self, key: &str) -> Result<Option<T>, ed::Error> {
        let key_name = T::key_name();
        let key_value = key.to_string();
        let query = doc!{key_name => key_value};
        self.find(&query).map(|mut vec| vec.pop())
    }

    fn update(&self, model: &T) -> Result<(), ed::Error> {
        let key_name = T::key_name();
        let key_value = model.key_value();
        let selector = doc!{key_name => key_value};
        let ret = self.client
            .update(&T::collection_name(), &selector, &model.to_doc());
        match ret {
            Ok(_) => Ok(()),
            Err(e) => match e {
                MongoError::Bsonc(e) => match e.code() {
                    MongoErrorCode::DuplicateKey => {
                        Err(ed::Error::with_chain(e, ed::ErrorKind::DuplicatedEntry))
                    }
                    _ => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ServerError(
                            "Unexpected error occurred when updating the document".to_string(),
                        ),
                    )),
                },
                _ => Err(ed::Error::with_chain(
                    e,
                    ed::ErrorKind::ServerError(
                        "Unexpected error occurred when updating the document".to_string(),
                    ),
                )),
            },
        }
    }

    fn upsert(&self, model: &T) -> Result<(), ed::Error> {
        let key_name = T::key_name();
        let key_value = model.key_value();
        let selector = doc!{key_name => key_value};
        let ret = self.client
            .upsert(&T::collection_name(), &selector, &model.to_doc());
        match ret {
            Ok(_) => Ok(()),
            Err(e) => match e {
                MongoError::Bsonc(e) => match e.code() {
                    MongoErrorCode::DuplicateKey => {
                        Err(ed::Error::with_chain(e, ed::ErrorKind::DuplicatedEntry))
                    }
                    _ => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ServerError(
                            "Unexpected error occurred when upserting the document".to_string(),
                        ),
                    )),
                },
                _ => Err(ed::Error::with_chain(
                    e,
                    ed::ErrorKind::ServerError(
                        "Unexpected error occurred when upserting the document".to_string(),
                    ),
                )),
            },
        }
    }

    fn remove(&self, model: &T) -> Result<(), ed::Error> {
        let key_name = T::key_name();
        let key_value = model.key_value();
        let query = doc!{key_name => key_value};
        self.client
            .remove(&T::collection_name(), &query)
            .chain_err(|| {
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when removing the document.".to_string(),
                )
            })
    }

    fn remove_by_key(&self, key: &str) -> Result<(), ed::Error> {
        let key_name = T::key_name();
        let key_value = key.to_string();
        let query = doc!{key_name => key_value};
        self.client
            .remove(&T::collection_name(), &query)
            .chain_err(|| {
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when removing the document.".to_string(),
                )
            })
    }
}
