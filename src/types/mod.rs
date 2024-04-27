


use std::collections::HashMap;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use crate::error;

pub type RustackiHttpResponse = Result<actix_web::HttpResponse, error::RustackiErrorResponse>;
pub type RamDb = std::sync::Arc<tokio::sync::Mutex<HashMap<String, String>>>;
pub type DbPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type LapinPoolConnection = deadpool::managed::Pool<deadpool_lapin::Manager>;
pub type RedisPoolConnection = deadpool::managed::Pool<deadpool_redis::Manager, deadpool_redis::Connection>;
