use std::sync::Arc;

use crate::{AppError, AppState};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Router;
use axum_extra::extract::CookieJar;

use super::AuthRequest;

const NAME: &str = "discord";
const AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const USER_API: &str = "https://discord.com/api/users/@me";

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", axum::routing::get(login))
        .route("/callback", axum::routing::get(callback))
}

pub async fn login() -> Result<impl IntoResponse, AppError> {
    super::login(NAME, AUTH_URL, TOKEN_URL)
}

pub async fn callback(
    cookies: CookieJar,
    query: Query<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    super::callback(cookies, query, NAME, AUTH_URL, TOKEN_URL, USER_API).await
}
