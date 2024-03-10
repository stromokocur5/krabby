pub mod components;
pub mod database;
pub mod errors;
pub mod routes;

pub use anyhow::{Context, Result};
pub use axum::async_trait;
pub use errors::AppError;
pub use routes::router;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

const SESSION_LENGTH: u32 = 1000 * 60 * 60 * 24;

#[derive(Clone)]
pub struct AppState {
    pub pg: PgPool,
    pub redis: RedisPool,
}

pub fn get_env(env: String) -> Result<String> {
    std::env::var(&env).context(format!("Missing the {} environment variable", env))
}
