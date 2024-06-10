use crate::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub files: Option<Vec<String>>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone)]
pub struct UserPost {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub avatar_url: String,
    pub content: String,
    pub files: Option<Vec<String>>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewPost {
    pub content: String,
    pub files: Option<Vec<Photo>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Photo {
    name: String,
    content: String,
}

impl Photo {
    // // async fn create(&self) -> Result<()> {
    //     let exists = tokio::fs::try_exists("uploads/").await?;
    //     if !exists {
    //         tokio::fs::create_dir("uploads/").await?;
    //     }
    //     Ok(())
    // }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeletePost {
    pub id: String,
}

impl Post {
    pub async fn create(user_id: &str, post: &NewPost, pg: &PgPool) -> Result<()> {
        // if let Some(files) = post.files.clone() {
        //     let files = files.as_slice();
        //     sqlx::query!(
        //         r#"
        //         INSERT INTO app_post (user_id,content,files) VALUES ($1,$2,$3);
        //     "#,
        //         user_id,
        //         post.content,
        //         files
        //     )
        //     .execute(pg)
        //     .await?;
        //     return Ok(());
        // }

        sqlx::query!(
            r#"
                INSERT INTO app_post (user_id,content) VALUES ($1,$2);
            "#,
            user_id,
            post.content,
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
    pub async fn get(id: &str, pg: &PgPool) -> Result<UserPost> {
        let post = sqlx::query_as!(
            UserPost,
            r#"
            SELECT app_post.id, app_user.id AS user_id, app_user.username as username,app_user.avatar_url as avatar_url, content, files, app_post.created_at 
            FROM app_post 
            JOIN app_user ON app_post.user_id = app_user.id 
            WHERE app_post.id = $1;
            "#,
            id
        )
        .fetch_one(pg)
        .await?;
        Ok(post)
    }
    pub async fn get_all(username: &str, pg: &PgPool) -> Result<Vec<UserPost>> {
        let posts = sqlx::query_as!(
            UserPost,
            r#"
            SELECT app_post.id, app_user.id AS user_id, app_user.username as username,app_user.avatar_url as avatar_url, content, files, app_post.created_at 
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
    pub async fn get_feed(pg: &PgPool) -> Result<Vec<UserPost>> {
        let posts = sqlx::query_as!(
            UserPost,
            r#"
            SELECT app_post.id, app_user.id AS user_id, app_user.username as username,app_user.avatar_url as avatar_url, content, files, app_post.created_at 
            FROM app_post 
            JOIN app_user ON app_post.user_id = app_user.id 
            ORDER BY app_post.created_at DESC
            LIMIT 100;
            "#,
        )
        .fetch_all(pg)
        .await?;
        Ok(posts)
    }
}
