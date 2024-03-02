use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::components::Base;
use crate::AppState;

pub mod api;
mod index;
mod login;
mod signup;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index::index))
        .merge(static_router())
        .nest("/api", api::router())
        .nest_service("/assets", ServeDir::new("assets"))
}

pub fn static_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", get(login::login))
        .route("/signup", get(signup::signup))
        .route_layer(middleware::from_fn(redirect_logged))
}

pub async fn redirect_logged(base: Base, req: Request, next: Next) -> Result<Response, StatusCode> {
    match base.logged_in {
        true => Ok(Redirect::to("/").into_response()),
        false => Ok(next.run(req).await),
    }
}
