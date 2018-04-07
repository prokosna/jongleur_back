use bson::{self, Bson, Document};
use serde::{Deserialize, Serialize};

/// `MongoModel` is the trait for models persisted in MongoDB.
/// The model implementing this trait can be used by `MongoClient`.
pub trait MongoModel: MongoSerializable {
    fn collection_name() -> String;
    fn key_name() -> String {
        "id".to_string()
    }
    fn key_value(&self) -> String;
    fn version_name() -> Option<String> {
        None
    }
    fn version_value(&self) -> Option<String> {
        None
    }
}

pub trait MongoSerializable {
    fn to_doc(&self) -> Document;
    fn from_doc(doc: &Document) -> Self;
}

impl<'a, T: Serialize + Deserialize<'a>> MongoSerializable for T {
    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}
