use crate::oauth_service;

const NAME: &str = "github";
const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const USER_API: &str = "https://api.github.com/user";

oauth_service!();
