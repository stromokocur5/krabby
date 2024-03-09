use crate::{database::LogInUser, Result};
use axum::response::IntoResponse;

use std::{net::Ipv4Addr, sync::Arc};

use axum::{
    extract::{Request, State},
    http::HeaderMap,
    response::Redirect,
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

use super::cloudflare;

pub async fn login(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Form(user): Form<LogInUser>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = User::verify(&user, &app_state.pg).await?;
    let ip: Ipv4Addr = headers
        .get("CF-Connecting-IP")
        .unwrap()
        .to_str()?
        .to_owned()
        .parse()?;
    cloudflare::verify_turnstitle(&user.cf_turnstile_response, ip.into()).await?;
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
