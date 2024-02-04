use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod auth;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/auth", auth::router())
}
