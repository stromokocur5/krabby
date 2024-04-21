use crate::database::OAuthUser;
use crate::oauth_service;
use crate::Result;

const NAME: &str = "discord";
const AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const USER_API: &str = "https://discord.com/api/users/@me";

pub async fn modify_user(user: OAuthUser) -> OAuthUser {
    let mut user = user.clone();
    user.avatar_url = format!(
        "https://cdn.discordapp.com/avatars/{}/{}.png",
        user.id, user.avatar_url
    );
    user
}

oauth_service!(modify_user);
