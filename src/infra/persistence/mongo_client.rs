use self::ed::ResultExt;
use bson::{Bson, Document};
use domain::error::domain as ed;
use infra::persistence::MongoModel;
use mongo_driver::client::ClientPool;
use mongo_driver::collection::{FindAndModifyOperation, UpdateOptions};
use mongo_driver::flags::{Flags, UpdateFlag};
use mongo_driver::{MongoError, MongoErrorCode};
use std::sync::Arc;

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

    pub fn update_if<T: MongoModel>(
        &self,
        cond: &mut Document,
        value: &T,
    ) -> Result<bool, ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name.clone());
        let key_name = T::key_name();
        let key_value = value.key_value();
        cond.insert(key_name, key_value);
        let doc = value.to_doc();
        let command = doc! {
            "update": coll_name,
            "q": Bson::Document(cond.clone()),
            "u": Bson::Document(doc)
        };
        let ret = coll.command_simple(command, None);
        match ret {
            Ok(v) => {
                let count = v.get("nModified").and_then(|i| i.as_i64()).unwrap_or(0i64);
                if count > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
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
                            "Unexpected error occurred when updating the document.".to_string(),
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

    pub fn update_with_version<T: MongoModel>(&self, value: &T) -> Result<bool, ed::Error> {
        let version_name = T::version_name();
        let version_value = value.version_value();
        if version_name.is_none() || version_value.is_none() {
            // This must be the programming error
            return Err(ed::ErrorKind::ServerError(
                "Unexpected error occurred when updating the document".to_string(),
            ).into());
        }
        let version_name = version_name.unwrap();
        let version_value = version_value.unwrap();

        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name.clone());
        let key_name = T::key_name();
        let key_value = value.key_value();
        let selector = doc! {
            key_name: key_value,
            version_name.clone(): version_value,
        };
        let mut doc = value.to_doc();
        doc.remove(&version_name);
        let command = doc! {
            "update": coll_name,
            "q": selector,
            "u": {
                "$set": Bson::Document(doc),
                "$inc": {
                    version_name.clone(): 1
                }
            }
        };
        let ret = coll.command_simple(command, None);
        match ret {
            Ok(v) => {
                let count = v.get("nModified").and_then(|i| i.as_i64()).unwrap_or(0i64);
                if count > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
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
                            "Unexpected error occurred when updating the document.".to_string(),
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

    pub fn remove<T: MongoModel>(&self, value: &T) -> Result<(), ed::Error> {
        let coll_name = T::collection_name();
        let client = self.client_pool.pop();
        let coll = client.get_collection(self.db_name.clone(), coll_name);
        let key_name = T::key_name();
        let key_value = value.key_value();
        let selector = doc! {key_name => key_value};
        let ret = coll.remove(&selector, None);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => Err(ed::Error::with_chain(
                e,
                ed::ErrorKind::ServerError(
                    "Unexpected error occurred when removing the document".to_string(),
                ),
            )),
        }
    }
}
