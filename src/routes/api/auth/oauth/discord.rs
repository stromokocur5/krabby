use std::sync::Arc;

use crate::{AppError, AppState};
use anyhow::Context;
use axum::response::IntoResponse;
use axum::Router;
use axum::{extract::Query, http::StatusCode, response::Redirect};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use oauth2::reqwest::async_http_client;
use oauth2::TokenResponse;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};

use super::{get_oauth_client, AuthRequest};

const NAME: &str = "discord";
const AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", axum::routing::get(login))
        .route("/callback", axum::routing::get(callback))
}

pub async fn login() -> Result<impl IntoResponse, AppError> {
    let client = get_oauth_client(NAME, AUTH_URL, TOKEN_URL)
        .context(format!("Failed to create {} auth client", NAME))?;
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let cookie_max_age = time::Duration::minutes(5);
    let csrf_cookie: Cookie = Cookie::build(("auth_csrf_state", csrf_state.secret().to_owned()))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(cookie_max_age.into())
        .into();

    let code_verifier: Cookie =
        Cookie::build(("auth_code_verifier", pkce_code_verifier.secret().to_owned()))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Lax)
            .max_age(cookie_max_age)
            .into();

    let cookies = CookieJar::new().add(csrf_cookie).add(code_verifier);

    Ok((cookies, Redirect::to(authorize_url.as_str())))
}

pub async fn callback(
    cookies: CookieJar,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let code = query.code;
    let state = query.state;
    let stored_state = cookies.get("auth_csrf_state");
    let stored_code_verifier = cookies.get("auth_code_verifier");

    let (Some(csrf_state), Some(code_verifier)) = (stored_state, stored_code_verifier) else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    if csrf_state.value() != state {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }

    let client = get_oauth_client(NAME, AUTH_URL, TOKEN_URL)
        .context(format!("Failed to create {} auth client", NAME))?;
    let code = AuthorizationCode::new(code);
    let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.value().to_owned());

    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .context("Failed to get token response")?;

    let discord_user = reqwest::Client::new()
        .get("https://discord.com/api/users/@me")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .context("Failed to get user info")?;

    // Add user session
    // let account_id = discord_user.id.clone();
    // let existing_user =
    // crate::db::get_user_by_account_id(&pool, AuthProvider::Discord, account_id.clone())
    // .await
    // .context("Failed to get user")?;

    // let user = match existing_user {
    // Some(x) => x,
    // None => crate::db::create_user(
    // &pool,
    // account_id.clone(),
    // ,
    // discord_user.username,
    // Some(format!(
    // "https://cdn.discordapp.com/avatars/{account_id}/{avatar_hash}.png",
    // avatar_hash = discord_user.avatar_hash
    // )),
    // )
    // .await
    // .context("Failed to create user")?,
    // };
    // let user_session = crate::db::create_user_session(&pool, user.id, SESSION_DURATION)
    // .await
    // .context("Failed to create user session")?;

    let mut remove_csrf_cookie = Cookie::new("auth_csrf_state", "");
    remove_csrf_cookie.set_path("/");
    remove_csrf_cookie.make_removal();

    let mut remove_code_verifier = Cookie::new("auth_code_verifier", "");
    remove_code_verifier.set_path("/");
    remove_code_verifier.make_removal();

    let session_cookie: Cookie = Cookie::build(("auth_session", "".to_string()))
        .same_site(SameSite::Lax)
        .http_only(true)
        .path("/")
        .max_age(time::Duration::milliseconds(1000 * 60 * 60 * 24))
        .into();

    let cookies = CookieJar::new()
        .add(remove_csrf_cookie)
        .add(remove_code_verifier)
        .add(session_cookie);

    Ok((cookies, Redirect::to("/")).into_response())
}
