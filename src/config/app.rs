use std::env;

pub struct AppConfig {}

impl AppConfig {
    // App
    pub fn issuer() -> String {
        env::var("ISSUER").unwrap()
    }
    pub fn default_admin_password() -> String {
        env::var("DEFAULT_ADMIN_PASSWORD").unwrap()
    }
    pub fn default_grant_max_age_sec() -> i64 {
        env::var("DEFAULT_GRANT_MAX_AGE_SEC")
            .map(|s| s.parse::<i64>().unwrap())
            .unwrap()
    }
    pub fn default_access_token_max_age_sec() -> i64 {
        env::var("DEFAULT_ACCESS_TOKEN_MAX_AGE_SEC")
            .map(|s| s.parse::<i64>().unwrap())
            .unwrap()
    }
    pub fn default_id_token_max_age_sec() -> i64 {
        env::var("DEFAULT_ID_TOKEN_MAX_AGE_SEC")
            .map(|s| s.parse::<i64>().unwrap())
            .unwrap()
    }
    pub fn default_refresh_token_max_age_sec() -> i64 {
        env::var("DEFAULT_REFRESH_TOKEN_MAX_AGE_SEC")
            .map(|s| s.parse::<i64>().unwrap())
            .unwrap()
    }

    // MongoDB
    pub fn mongo_db() -> String {
        env::var("MONGO_DB").unwrap()
    }
    pub fn mongo_endpoint() -> String {
        env::var("MONGO_ENDPOINT").unwrap()
    }

    // Redis
    pub fn redis_endpoint() -> String {
        env::var("REDIS_ENDPOINT").unwrap()
    }
    pub fn redis_expires_sec() -> u64 {
        env::var("REDIS_EXPIRES_SEC")
            .map(|s| s.parse::<u64>().unwrap())
            .unwrap()
    }

    // Keys
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
