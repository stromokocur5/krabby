use crate::database::user::User;
use crate::database::OAuthUser;
use crate::oauth_service;
use crate::Result;

const NAME: &str = "discord";
const AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const USER_API: &str = "https://discord.com/api/users/@me";

pub async fn func(app_state: Arc<AppState>, user: OAuthUser) -> Result<(String, String)> {
    let mut user = user.clone();
    user.avatar_url = format!(
        "https://cdn.discordapp.com/avatars/{}/{}.png",
        user.id, user.avatar_url
    );
    let user_id = User::oauth_create(user, NAME, &app_state.pg).await?;
    let session_id = User::create_session(&user_id, &app_state.redis).await?;

    tracing::debug!(user_id, session_id);
    Ok((user_id, session_id))
}

oauth_service!(func);
