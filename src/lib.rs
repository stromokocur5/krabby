pub mod components;
mod routes;

pub use routes::router;

use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pg: PgPool,
    pub redis: RedisPool,
}
