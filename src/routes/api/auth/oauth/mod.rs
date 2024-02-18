use std::sync::Arc;

use crate::{get_env, AppState, Context, Result};
use axum::Router;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub mod discord;
pub mod github;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/discord", discord::router())
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
