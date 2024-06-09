use std::sync::Arc;

use axum::{
    http::HeaderMap,
    routing::{get, post},
    Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};

use crate::{database::User, AppError, AppState};

use cloudflare::{get_ip, verify_turnstitle};

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
pub async fn get_auth_cookies(
    headers: &HeaderMap,
    user_id: &str,
    cf_turnstile_response: &str,
    app_state: &AppState,
) -> Result<CookieJar, AppError> {
    let ip = get_ip(headers).await?;

    verify_turnstitle(cf_turnstile_response, ip.into()).await?;

    let session_id = User::create_session(&user_id, &app_state.redis).await?;

    let user_id_cookie: Cookie = Cookie::build(("user_id", user_id.to_owned()))
        .same_site(SameSite::Lax)
        .http_only(true)
        .path("/")
        .max_age(time::Duration::milliseconds(crate::SESSION_LENGTH.into()))
        .into();

    let session_id_cookie: Cookie = Cookie::build(("session_id", session_id))
        .same_site(SameSite::Lax)
        .http_only(true)
        .path("/")
        .max_age(time::Duration::milliseconds(crate::SESSION_LENGTH.into()))
        .into();

    let cookies = CookieJar::new().add(user_id_cookie).add(session_id_cookie);
    Ok(cookies)
}
