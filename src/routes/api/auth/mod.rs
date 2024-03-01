use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod cloudflare;
pub mod login;
pub mod logout;
pub mod middleware;
pub mod oauth;
pub mod register;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(oauth::router())
}
