use std::sync::Arc;

use axum::{middleware, Router};

use crate::AppState;

mod auth;
mod post;
mod update;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::router())
        .merge(authorized_only())
}

fn authorized_only() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/post", post::router())
        .nest("/update", update::router())
        .layer(middleware::from_fn(
            crate::routes::middleware::only_authorized,
        ))
}
