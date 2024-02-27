use std::sync::Arc;

use crate::database::OAuthUser;
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

#[macro_export]
macro_rules! oauth_service {
    () => {
        use std::sync::Arc;

        use crate::{AppError, AppState};
        use axum::extract::{Query, State};
        use axum::response::IntoResponse;
        use axum::Router;
        use axum_extra::extract::CookieJar;

        use super::AuthRequest;

        pub fn router() -> Router<Arc<AppState>> {
            Router::new()
                .route("/login", axum::routing::get(login))
                .route("/callback", axum::routing::get(callback))
        }

        pub async fn login() -> Result<impl IntoResponse, AppError> {
            super::login(NAME, AUTH_URL, TOKEN_URL)
        }

        pub async fn callback(
            State(app_state): State<Arc<AppState>>,
            cookies: CookieJar,
            query: Query<AuthRequest>,
        ) -> Result<impl IntoResponse, AppError> {
            super::callback(
                app_state, cookies, query, NAME, AUTH_URL, TOKEN_URL, USER_API,
            )
            .await
        }
    };
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/discord", discord::router())
        .nest("/github", github::router())
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
        .add_scope(Scope::new("email".to_string()))
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
    app_state: Arc<AppState>,
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

    let user = reqwest::Client::new()
        .get(user_api)
        .header("User-Agent", "Rust")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .context("Failed to get user info")?
        .json::<OAuthUser>()
        .await?;

    let oauth_type = format!("{}_id", name);

    let query = format!(
        "
        INSERT INTO app_user ({}, username, email, password_hash, avatar_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, avatar_url;
        ",
        oauth_type
    );
    let users = sqlx::query_as::<_, OAuthUser>(&query)
        .bind(user.id)
        .bind(user.username)
        .bind("test_email")
        .bind("test_heslo")
        .bind(user.avatar_url)
        .fetch_one(&app_state.pg)
        .await?;
    println!("{:?}", users);

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

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
