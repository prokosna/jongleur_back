use bson::Document;
use mongo_driver::client::ClientPool;
use mongo_driver::collection::{FindAndModifyOperation, UpdateOptions};
use mongo_driver::flags::{Flags, UpdateFlag};
use mongo_driver::{MongoError, MongoErrorCode};
use std::sync::Arc;

use self::ed::ResultExt;
use domain::error::domain as ed;
use infra::persistence::MongoModel;

/// `MongoClient` is the common Mongo client for repositories.
#[derive(Clone)]
pub struct MongoClient {
    db_name: String,
    client_pool: Arc<ClientPool>,
}

impl MongoClient {
    pub fn new(db_name: &String, client_pool: Arc<ClientPool>) -> Self {
        MongoClient {
            db_name: db_name.clone(),
            client_pool: client_pool.clone(),
        }
    }

    pub fn find<T: MongoModel>(&self, query: &Document) -> Result<Vec<T>, ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name);
        let ret = coll.find(query, None);
        ret.map(|cursor| {
            cursor
                .filter_map(|doc| match doc {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
                .collect::<Vec<Document>>()
        }).map(|v| v.iter().map(|doc| T::from_doc(&doc)).collect::<Vec<T>>())
            .chain_err(|| {
                ed::ErrorKind::ServerError(format!(
                    "Unexpected error occurred when finding the documents. {}",
                    query
                ))
            })
    }

    pub fn insert<T: MongoModel>(&self, value: &T) -> Result<(), ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name);
        let doc = value.to_doc();
        let ret = coll.insert(&doc, None);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => match e {
                MongoError::Bsonc(e) => match e.code() {
                    MongoErrorCode::DuplicateKey => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::DuplicatedEntity(
                            "The document who has the same unique values exists.".to_string(),
                        ),
                    )),
                    _ => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ServerError(
                            "Unexpected error occurred when inserting the document.".to_string(),
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

    pub fn update<T: MongoModel>(&self, value: &T) -> Result<(), ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name);
        let key_name = T::key_name();
        let key_value = value.key_value();
        let selector = doc! {key_name => key_value};
        let doc = value.to_doc();
        let ret = coll.update(&selector, &doc, None);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => match e {
                MongoError::Bsonc(e) => match e.code() {
                    MongoErrorCode::DuplicateKey => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ConflictDetected(
                            "Some unique values are conflicting with another document.".to_string(),
                        ),
                    )),
                    _ => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ServerError(
                            "Unexpected error occurred when inserting the document.".to_string(),
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

    pub fn upsert<T: MongoModel>(&self, value: &T) -> Result<(), ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name);
        let key_name = T::key_name();
        let key_value = value.key_value();
        let selector = doc! {key_name => key_value};
        let doc = value.to_doc();
        let mut opts = UpdateOptions::default();
        opts.update_flags = Flags::new();
        opts.update_flags.add(UpdateFlag::Upsert);
        let ret = coll.update(&selector, &doc, Some(&opts));
        match ret {
            Ok(_) => Ok(()),
            Err(e) => match e {
                MongoError::Bsonc(e) => match e.code() {
                    MongoErrorCode::DuplicateKey => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ConflictDetected(
                            "Some unique values are conflicting with another document.".to_string(),
                        ),
                    )),
                    _ => Err(ed::Error::with_chain(
                        e,
                        ed::ErrorKind::ServerError(
                            "Unexpected error occurred when inserting the document.".to_string(),
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

    pub fn find_and_modify<T: MongoModel>(
        &self,
        query: &Document,
        modify: &Document,
    ) -> Result<Option<T>, ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name);
        let operation = FindAndModifyOperation::Update(&modify);
        let ret = coll.find_and_modify(query, operation, None);
        ret.map(|doc| match doc.get_document("value") {
            Ok(v) => Some(T::from_doc(&v)),
            Err(_) => None,
        }).chain_err(|| {
            ed::ErrorKind::ServerError(format!(
                "Unexpected error occurred when finding the documents. {}",
                query
            ))
        })
    }
}
