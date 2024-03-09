use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

pub mod cloudflare;
pub mod login;
pub mod logout;
pub mod oauth;
pub mod signup;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(oauth::router())
        .route("/login", post(login::login))
        .route("/signup", post(signup::signup))
        .route("/logout", get(logout::logout))
}
