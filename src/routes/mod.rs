use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::AppState;
pub use not_found::NotFound;

mod api;
mod auth;
mod index;
pub mod middleware;
pub mod not_found;
mod users;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index::index))
        .merge(auth::router())
        .nest("/users", users::router())
        .nest("/api", api::router())
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/*fallback", get(not_found::not_found))
}
