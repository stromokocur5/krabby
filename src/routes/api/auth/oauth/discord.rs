use crate::oauth_service;
use crate::Result;

const NAME: &str = "discord";
const AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const USER_API: &str = "https://discord.com/api/users/@me";

pub fn func(app_state: Arc<AppState>) -> Result<String> {
    Ok("".to_string())
}

oauth_service!(func);
