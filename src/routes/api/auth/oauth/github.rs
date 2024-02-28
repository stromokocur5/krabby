use crate::database::user::User;
use crate::database::OAuthUser;
use crate::oauth_service;
use crate::Result;

const NAME: &str = "github";
const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const USER_API: &str = "https://api.github.com/user";

pub async fn func(app_state: Arc<AppState>, user: OAuthUser) -> Result<String> {
    User::oauth_create(user, NAME, &app_state.pg).await?;
    Ok("".to_string())
}

oauth_service!(func);
