use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod oauth;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(oauth::router())
}

#[derive(Debug, PartialEq)]
pub enum AuthProvider {
    Database,
    OAuth,
}
