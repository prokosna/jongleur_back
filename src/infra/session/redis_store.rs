use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::{self, ErrorKind};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use config::AppConfig;
use domain::error::domain as ed;
use infra::session::RedisClient;
use self::ed::ResultExt;

pub type Pool = r2d2::Pool<RedisConnectionManager>;
pub type Connection = r2d2::PooledConnection<RedisConnectionManager>;

pub struct RedisStore {
    client: RedisClient,
}

impl RedisStore {
    pub fn new(redis_client: RedisClient) -> Self {
        RedisStore {
            client: redis_client,
        }
    }
    pub fn set(&self, key: &str, field: &str, value: &str) -> Result<(), ed::Error> {
        let mut pipe = redis::pipe();
        let p = pipe.cmd("HSET")
            .arg(key)
            .arg(field)
            .arg(value)
            .ignore()
            .cmd("EXPIRE")
            .arg(key)
            .arg(AppConfig::redis_expires_sec());
        self.client
            .query_pipeline::<()>(p)
            .chain_err(|| ed::ErrorKind::ServerError("Setting value to Redis failed.".to_string()))
    }

    pub fn get(&self, key: &str, field: &str) -> Result<Option<String>, ed::Error> {
        let mut cmd = redis::cmd("HGET");
        let c = cmd.arg(key).arg(field);
        let ret = self.client.query_cmd::<String>(c);
        match ret {
            Ok(s) => Ok(Some(s)),
            Err(e) => match e.kind() {
                ErrorKind::TypeError => Ok(None),
                _ => Err(ed::Error::with_chain(
                    e,
                    ed::ErrorKind::ServerError("Getting value from Redis failed.".to_string()),
                )),
            },
        }
    }

    pub fn del(&self, key: &str, field: Option<&str>) -> Result<(), ed::Error> {
        let mut pipe = redis::pipe();
        let p;
        match field {
            Some(_) => p = pipe.cmd("HDEL").arg(key).arg(field),
            None => p = pipe.cmd("DEL").arg(key),
        };
        self.client.query_pipeline::<()>(p).chain_err(|| {
            ed::ErrorKind::ServerError("Deleting value from Redis failed.".to_string())
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RedisStore {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisStore::new(RedisClient(conn))),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
