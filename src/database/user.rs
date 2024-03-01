use crate::Result;
use anyhow::anyhow;
use chrono::DateTime;
use deadpool_redis::redis::cmd as redis_cmd;
use deadpool_redis::Pool as RedisPool;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, sqlx::FromRow, Clone)]
pub struct BaseUser {
    pub username: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct OAuthUser {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub id: String,
    #[serde(alias = "login")]
    pub username: String,
    #[serde(alias = "avatar")]
    pub avatar_url: String,
}

#[derive(Deserialize)]
pub struct SignUpUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    #[serde(alias = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
}

#[derive(Deserialize)]
pub struct LogInUser {
    pub username: String,
    pub password: String,
    #[serde(alias = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone)]
pub struct User {
    pub id: String,
    pub discord_id: Option<String>,
    pub github_id: Option<String>,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<chrono::Local>,
}

impl User {
    pub async fn create(user: SignUpUser, pg: &PgPool) -> Result<String> {
        Ok("".into())
    }
    pub async fn oauth_create(user: OAuthUser, name: &str, pg: &PgPool) -> Result<String> {
        #[derive(sqlx::FromRow)]
        struct Id {
            id: String,
        }

        let oauth_type = format!("{}_id", name);
        let mut user = user.clone();

        if let Ok(_) = User::exists(&oauth_type, &user.id, pg).await {
            tracing::debug!("have an account");
            let query = format!("SELECT id FROM app_user WHERE {} = $1;", oauth_type);

            let user_id = sqlx::query_as::<_, Id>(&query)
                .bind(user.id.clone())
                .fetch_one(pg)
                .await?;
            tracing::debug!(?user);
            return Ok(user_id.id);
        }

        if let Ok(_) = User::exists("username", &user.username, pg).await {
            tracing::debug!("username collision");
            let rand1 = rand::random::<u8>();
            let rand2 = rand::random::<u8>();
            user.username = format!("{}{}{}", rand1, user.username, rand2);
        }

        let query = format!(
            "
            INSERT INTO app_user ({}, username, avatar_url)
            VALUES ($1, $2, $3)
            RETURNING id;
            ",
            oauth_type
        );

        let user_id = sqlx::query_as::<_, Id>(&query)
            .bind(user.id.clone())
            .bind(user.username.clone())
            .bind(user.avatar_url.clone())
            .fetch_one(pg)
            .await?;
        tracing::debug!(?user);
        Ok(user_id.id)
    }
    pub async fn get(user_id: &str, pg: &PgPool) -> Result<()> {
        Ok(())
    }

    pub async fn get_base(user_id: &str, pg: &PgPool) -> Result<BaseUser> {
        let user = sqlx::query_as!(
            BaseUser,
            r#"
            SELECT username,avatar_url FROM app_user where id = $1;
            "#,
            user_id
        )
        .fetch_one(pg)
        .await?;
        Ok(user)
    }
    pub async fn update(user_id: &str, pg: &PgPool) -> Result<()> {
        Ok(())
    }
    pub async fn delete(user_id: &str, pg: &PgPool) -> Result<()> {
        Ok(())
    }
    pub async fn exists(key: &str, value: &str, pg: &PgPool) -> Result<()> {
        #[derive(sqlx::FromRow, Debug)]
        struct Exists {
            exists: Option<bool>,
        }

        let query = sqlx::query_as::<_, Exists>(
            format!("SELECT EXISTS (SELECT 1 FROM app_user WHERE {} = $1)", key).as_str(),
        )
        .bind(value)
        .fetch_one(pg)
        .await?;
        tracing::debug!(?query, key);
        match query.exists {
            Some(true) => Ok(()),
            Some(false) => Err(anyhow!(format!("{} does not exist", key))),
            None => Err(anyhow!("something went wrong")),
        }
    }
    pub async fn create_session(user_id: &str, redis: &RedisPool) -> Result<String> {
        let mut redis = redis.get().await?;
        let session_id = uuid::Builder::from_random_bytes(rand::random())
            .into_uuid()
            .to_string();
        redis_cmd("SADD")
            .arg(user_id)
            .arg(session_id.clone())
            .query_async(&mut redis)
            .await?;
        redis_cmd("EXPIRE")
            .arg(user_id)
            .arg(crate::SESSION_LENGTH)
            .query_async(&mut redis)
            .await?;
        Ok(session_id)
    }
    pub async fn verify_session(user_id: &str, session_id: &str, redis: &RedisPool) -> Result<()> {
        let mut redis = redis.get().await?;
        let result: u8 = redis_cmd("SISMEMBER")
            .arg(user_id)
            .arg(session_id)
            .query_async(&mut redis)
            .await?;
        match result {
            1 => Ok(()),
            _ => Err(anyhow!("not valid session id")),
        }
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
