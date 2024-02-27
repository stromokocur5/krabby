use crate::Result;
use chrono::DateTime;
use deadpool_redis::Pool as RedisPool;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct OAuthUser {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub id: String,
    #[serde(alias = "login")]
    pub username: String,
    // pub email: String,
    #[serde(alias = "avatar")]
    pub avatar_url: String,
}

pub struct SignUpUser {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
pub struct LogInUser;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub discord_id: Option<String>,
    pub github_id: Option<String>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: String,
    pub created_at: DateTime<chrono::Local>,
}

impl User {
    async fn verify_session(user_id: String, session_id: String, redis: &RedisPool) -> Result<()> {
        Ok(())
    }
    async fn create(user: SignUpUser, pool: &PgPool) -> Result<()> {
        Ok(())
    }
    async fn delete(user_id: String, pool: &PgPool) -> Result<()> {
        Ok(())
    }
    async fn get(user_id: String, pool: &PgPool) -> Result<()> {
        Ok(())
    }
    async fn update(user_id: String, pool: &PgPool) -> Result<()> {
        Ok(())
    }
}

pub fn deserialize_string_from_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Float(f64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(s),
        StringOrNumber::Number(i) => Ok(i.to_string()),
        StringOrNumber::Float(f) => Ok(f.to_string()),
    }
}
