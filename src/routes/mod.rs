use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::services::ServeDir;

pub mod index;
pub mod login;
pub mod signup;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::index))
        .nest_service("/assets", ServeDir::new("assets"))
}
