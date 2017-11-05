use bson::Document;
use mongo_driver::client::{Client, ClientPool, Uri};
use mongo_driver::flags::{Flags, UpdateFlag};
use mongo_driver::collection::UpdateOptions;
use mongo_driver::MongoError;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};

use util::DomainConfig;

pub struct MongoClient<'a>(pub Client<'a>);

impl<'a> MongoClient<'a> {
    pub fn init_pool() -> ClientPool {
        let uri = Uri::new(DomainConfig::mongo_endpoint().to_string()).unwrap();
        ClientPool::new(uri.clone(), None)
    }

    pub fn insert(&self, collection: &str, doc: &Document) -> Result<(), MongoError> {
        let coll = self.0
            .get_collection(DomainConfig::mongo_db().to_string(), collection);
        coll.insert(doc, None)
    }

    pub fn update(
        &self,
        collection: &str,
        selector: &Document,
        doc: &Document,
    ) -> Result<(), MongoError> {
        let coll = self.0
            .get_collection(DomainConfig::mongo_db().to_string(), collection);
        coll.update(selector, doc, None)
    }

    pub fn upsert(
        &self,
        collection: &str,
        selector: &Document,
        doc: &Document,
    ) -> Result<(), MongoError> {
        let coll = self.0
            .get_collection(DomainConfig::mongo_db().to_string(), collection);
        let mut update_opts = UpdateOptions::default();
        update_opts.update_flags = Flags::new();
        update_opts.update_flags.add(UpdateFlag::Upsert);
        coll.update(selector, doc, Some(&update_opts))
    }

    pub fn find(&self, collection: &str, query: &Document) -> Result<Vec<Document>, MongoError> {
        let coll = self.0
            .get_collection(DomainConfig::mongo_db().to_string(), collection);
        let ret = coll.find(query, None);
        ret.map(|cursor| {
            cursor
                .filter_map(|item| match item {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
                .collect()
        })
    }

    pub fn remove(&self, collection: &str, query: &Document) -> Result<(), MongoError> {
        let coll = self.0
            .get_collection(DomainConfig::mongo_db().to_string(), collection);
        coll.remove(&query, None)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for MongoClient<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request
            .guard::<State<ClientPool>>()
            .map(|pool| MongoClient(pool.inner().pop()))
    }
}
