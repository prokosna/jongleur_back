use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::{Cmd, FromRedisValue, Pipeline, RedisError};

use config::AppConfig;

pub type Pool = r2d2::Pool<RedisConnectionManager>;
pub type Connection = r2d2::PooledConnection<RedisConnectionManager>;

pub struct RedisClient(pub Connection);

impl RedisClient {
    pub fn init_pool() -> Pool {
        let manager =
            RedisConnectionManager::new(&*AppConfig::redis_endpoint().to_string()).unwrap();
        r2d2::Pool::new(manager).unwrap()
    }

    pub fn query_cmd<T: FromRedisValue>(&self, cmd: &Cmd) -> Result<T, RedisError> {
        cmd.query::<T>(&*self.0)
    }

    pub fn query_pipeline<T: FromRedisValue>(&self, pipeline: &Pipeline) -> Result<T, RedisError> {
        pipeline.query::<T>(&*self.0)
    }
}

//impl<'a, 'r> FromRequest<'a, 'r> for RedisClient {
//    type Error = ();
//
//    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
//        let pool = request.guard::<State<Pool>>()?;
//        match pool.get() {
//            Ok(conn) => Outcome::Success(RedisClient(conn)),
//            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//        }
//    }
//}
