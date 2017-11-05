use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::{self, ErrorKind};

use domain::consts;
use domain::session::Store;
use infra::session::RedisClient;
use domain::error::domain as ed;
use self::ed::ResultExt;

pub type Pool = r2d2::Pool<RedisConnectionManager>;
pub type Connection = r2d2::PooledConnection<RedisConnectionManager>;

pub struct RedisStore<'a> {
    client: &'a RedisClient,
}

impl<'a> RedisStore<'a> {
    pub fn new(redis_client: &'a RedisClient) -> Self {
        RedisStore {
            client: redis_client,
        }
    }
}

impl<'a> Store for RedisStore<'a> {
    fn set(&self, key: &str, field: &str, value: &str) -> Result<(), ed::Error> {
        let mut pipe = redis::pipe();
        let p = pipe.cmd("HSET")
            .arg(key)
            .arg(field)
            .arg(value)
            .ignore()
            .cmd("EXPIRE")
            .arg(key)
            .arg(consts::REDIS_EXPIRES_SEC);
        self.client.query_pipeline::<()>(p).chain_err(|| {
            ed::ErrorKind::ServerError("Setting value to Redis failed.".to_string())
        })
    }

    fn get(&self, key: &str, field: &str) -> Result<String, ed::Error> {
        let mut cmd = redis::cmd("HGET");
        let c = cmd.arg(key).arg(field);
        let ret = self.client.query_cmd::<String>(c);
        match ret {
            Ok(s) => Ok(s),
            Err(e) => match e.kind() {
                ErrorKind::TypeError => Err(ed::Error::with_chain(e, ed::ErrorKind::RequireLogin)),
                _ => Err(ed::Error::with_chain(
                    e,
                    ed::ErrorKind::ServerError("Getting value from Redis failed.".to_string()),
                )),
            },
        }
    }

    fn del(&self, key: &str, field: Option<&str>) -> Result<(), ed::Error> {
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
