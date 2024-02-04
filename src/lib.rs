pub mod components;
pub mod database;
mod routes;

pub use anyhow::{Context, Result};
pub use axum::async_trait;
pub use routes::router;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pg: PgPool,
    pub redis: RedisPool,
}

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub fn get_env(env: String) -> Result<String> {
    std::env::var(&env).context(format!("Missing the {} environment variable", env))
}
