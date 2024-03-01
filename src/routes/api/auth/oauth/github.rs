use crate::database::user::User;
use crate::database::OAuthUser;
use crate::oauth_service;
use crate::Result;

const NAME: &str = "github";
const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const USER_API: &str = "https://api.github.com/user";

pub async fn func(app_state: Arc<AppState>, user: OAuthUser) -> Result<(String, String)> {
    let user_id = User::oauth_create(user, NAME, &app_state.pg).await?;
    let session_id = User::create_session(&user_id, &app_state.redis).await?;

    tracing::debug!(user_id, session_id);
    Ok((user_id, session_id))
}

oauth_service!(func);
