use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::{self, Cmd, ErrorKind, FromRedisValue, Pipeline, RedisError};

use self::ed::ResultExt;
use config::AppConfig;
use domain::error::domain as ed;

pub type Pool = r2d2::Pool<RedisConnectionManager>;
pub type Connection = r2d2::PooledConnection<RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisPool {
    pool: Pool,
}

impl RedisPool {
    pub fn new() -> Self {
        RedisPool {
            pool: RedisClient::init_pool(),
        }
    }

    pub fn get_store(&self) -> Result<RedisStore, ed::Error> {
        self.pool
            .get()
            .map(|conn| RedisStore {
                client: RedisClient(conn),
            })
            .chain_err(|| {
                ed::ErrorKind::TemporarilyUnavailable("Redis cluster is not ready".to_string())
            })
    }
}

pub struct RedisClient(pub Connection);

impl RedisClient {
    fn init_pool() -> Pool {
        let manager =
            RedisConnectionManager::new(&*AppConfig::redis_endpoint().to_string()).unwrap();
        r2d2::Pool::new(manager).unwrap()
    }

    fn query_cmd<T: FromRedisValue>(&self, cmd: &Cmd) -> Result<T, RedisError> {
        cmd.query::<T>(&*self.0)
    }

    fn query_pipeline<T: FromRedisValue>(&self, pipeline: &Pipeline) -> Result<T, RedisError> {
        pipeline.query::<T>(&*self.0)
    }
}

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
