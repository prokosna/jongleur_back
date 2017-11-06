use std::env;

pub struct DomainConfig {}

impl DomainConfig {
    pub fn mongo_db() -> String {
        env::var("MONGO_DB").unwrap()
    }
    pub fn mongo_endpoint() -> String {
        env::var("MONGO_ENDPOINT").unwrap()
    }
    pub fn redis_endpoint() -> String {
        env::var("REDIS_ENDPOINT").unwrap()
    }
    pub fn jwt_private_key() -> String {
        env::var("JWT_PRIVATE_KEY").unwrap()
    }
    pub fn jwt_public_key() -> String {
        env::var("JWT_PUBLIC_KEY").unwrap()
    }
    pub fn jwt_public_key_pem() -> String {
        env::var("JWT_PUBLIC_KEY_PEM").unwrap()
    }
}
