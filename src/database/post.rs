use crate::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewPost {
    pub content: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeletePost {
    pub id: String,
}

impl Post {
    pub async fn create(user_id: &str, content: &str, pg: &PgPool) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO app_post (user_id,content) VALUES ($1,$2);
            "#,
            user_id,
            content
        )
        .execute(pg)
        .await?;
        Ok(())
    }
    pub async fn delete(user_id: &str, id: &str, pg: &PgPool) -> Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM app_post WHERE user_id=$1 AND id=$2;
            "#,
            user_id,
            id
        )
        .execute(pg)
        .await?;
        Ok(())
    }
    pub async fn get(id: &str, pg: &PgPool) -> Result<()> {
        Ok(())
    }
    pub async fn get_all(username: &str, pg: &PgPool) -> Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT app_post.id, app_user.id AS user_id, content, app_post.created_at 
            FROM app_post 
            JOIN app_user ON app_post.user_id = app_user.id 
            WHERE app_user.username = $1 ORDER BY app_post.created_at DESC;
            "#,
            username
        )
        .fetch_all(pg)
        .await?;
        Ok(posts)
    }
}
