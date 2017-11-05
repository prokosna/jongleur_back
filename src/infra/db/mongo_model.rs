use bson::{self, Bson, Document};

use domain::model::{AccessToken, Client, EndUser, Grant, IdToken, RefreshToken, Resource};

pub trait MongoModel {
    fn collection_name() -> String;
    fn key_name() -> String {
        "_id".to_string()
    }
    fn key_value(&self) -> String;
    fn to_doc(&self) -> Document;
    fn from_doc(doc: &Document) -> Self;
}

impl MongoModel for AccessToken {
    fn collection_name() -> String {
        "access_tokens".to_string()
    }

    fn key_value(&self) -> String {
        self.id.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}

impl MongoModel for Client {
    fn collection_name() -> String {
        "clients".to_string()
    }

    fn key_value(&self) -> String {
        self.id.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}

impl MongoModel for Resource {
    fn collection_name() -> String {
        "resources".to_string()
    }

    fn key_value(&self) -> String {
        self.id.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}

impl MongoModel for EndUser {
    fn collection_name() -> String {
        "end_users".to_string()
    }

    fn key_value(&self) -> String {
        self.id.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}

impl MongoModel for Grant {
    fn collection_name() -> String {
        "grants".to_string()
    }

    fn key_value(&self) -> String {
        self.id.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}

impl MongoModel for IdToken {
    fn collection_name() -> String {
        "id_tokens".to_string()
    }

    fn key_value(&self) -> String {
        self.id.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}

impl MongoModel for RefreshToken {
    fn collection_name() -> String {
        "refresh_tokens".to_string()
    }

    fn key_value(&self) -> String {
        self.token.clone()
    }

    fn to_doc(&self) -> Document {
        let b = bson::to_bson(&self).unwrap();
        b.as_document().unwrap().clone()
    }

    fn from_doc(doc: &Document) -> Self {
        bson::from_bson(Bson::Document(doc.clone())).unwrap()
    }
}
