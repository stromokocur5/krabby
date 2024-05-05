use std::sync::Arc;

use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};

use crate::{
    database::{SignUpUser, User},
    AppError, AppState,
};

use super::cloudflare::{get_ip, verify_turnstitle};

pub async fn signup(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Form(user): Form<SignUpUser>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = User::create(&user, &app_state.pg).await?;
    let ip = get_ip(&headers).await?;
    verify_turnstitle(&user.cf_turnstile_response, ip.into()).await?;
    let session_id = User::create_session(&user_id, &app_state.redis).await?;
    let user_id_cookie: Cookie = Cookie::build(("user_id", user_id))
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

    Ok((cookies, Redirect::to("/")).into_response())
}
