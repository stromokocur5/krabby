use std::sync::Arc;

use crate::{get_env, AppError, AppState, Context, Result};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};

pub mod discord;
pub mod github;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/discord", discord::router())
        .nest("/github", github::router())
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

pub fn get_oauth_client(name: &str, auth_url: &str, token_url: &str) -> Result<BasicClient> {
    let name_uppercase = name.to_uppercase();
    let client_id = ClientId::new(get_env(format!("{}_CLIENT_ID", name_uppercase))?);

    let client_secret = ClientSecret::new(get_env(format!("{}_CLIENT_SECRET", name_uppercase))?);

    let auth_url = AuthUrl::new(auth_url.into()).context("Invalid authorization endpoint URL")?;
    let token_url = TokenUrl::new(token_url.into()).context("Invalid token endpoint URL")?;

    let base_url = std::env::var("BASE_URL").context("Failed to get app base url")?;
    let redirect_url = RedirectUrl::new(format!("{}/api/auth/{}/callback", base_url, name))
        .context("Invalid redirect url")?;

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    Ok(client)
}

pub fn login(name: &str, auth_url: &str, token_url: &str) -> Result<impl IntoResponse, AppError> {
    let client = get_oauth_client(name, auth_url, token_url)
        .context(format!("Failed to create {} auth client", name))?;
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
    name: &str,
    auth_url: &str,
    token_url: &str,
    user_api: &str,
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

    let client = get_oauth_client(name, auth_url, token_url)
        .context(format!("Failed to create {} auth client", name))?;
    let code = AuthorizationCode::new(code);
    let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.value().to_owned());

    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .context("Failed to get token response")?;

    let _user = reqwest::Client::new()
        .get(user_api)
        .header("User-Agent", "Rust")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .context("Failed to get user info")?;

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
