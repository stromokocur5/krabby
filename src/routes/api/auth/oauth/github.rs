use crate::oauth_service;
use crate::Result;

const NAME: &str = "github";
const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const USER_API: &str = "https://api.github.com/user";

pub fn func(app_state: Arc<AppState>) -> Result<String> {
    Ok("".to_string())
}

oauth_service!(func);
