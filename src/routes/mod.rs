use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::AppState;

mod api;
mod index;
mod login;
mod signup;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index::index))
        .route("/login", get(login::login))
        .route("/signup", get(signup::signup))
        .nest("/api", api::router())
        .nest_service("/assets", ServeDir::new("assets"))
}
