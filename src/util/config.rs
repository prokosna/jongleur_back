use config::{self, Config};
use std::collections::HashMap;

lazy_static! {
  static ref CONFIG: HashMap<String, String> = {
    let mut conf = Config::default();
    conf
        .merge(config::File::with_name("Config")).unwrap()
        .merge(config::Environment::with_prefix("JL")).unwrap();
    conf.try_into::<HashMap<String, String>>().unwrap()
  };
}

pub struct DomainConfig {}

impl DomainConfig {
    pub fn mongo_db<'a>() -> &'a String {
        CONFIG.get("mongo_db").unwrap()
    }
    pub fn mongo_endpoint<'a>() -> &'a String {
        CONFIG.get("mongo_endpoint").unwrap()
    }
    pub fn redis_endpoint<'a>() -> &'a String {
        CONFIG.get("redis_endpoint").unwrap()
    }
    pub fn jwt_private_key<'a>() -> &'a String {
        CONFIG.get("jwt_private_key").unwrap()
    }
    pub fn jwt_public_key<'a>() -> &'a String {
        CONFIG.get("jwt_public_key").unwrap()
    }
    pub fn jwt_public_key_pem<'a>() -> &'a String {
        CONFIG.get("jwt_public_key_pem").unwrap()
    }
}
