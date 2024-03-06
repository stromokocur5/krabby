use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{
    components::Base,
    database::{Post, User},
    AppState,
};

use super::NotFound;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/:username", get(user_profile))
}
#[derive(Template)]
#[template(path = "routes/profile.html")]
struct UserProfile {
    base: Base,
    user: User,
    posts: Vec<Post>,
}
async fn user_profile(
    State(app_state): State<Arc<AppState>>,
    Path(username): Path<String>,
    base: Base,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user = User::get(format!("username='{username}'").as_str(), &app_state.pg).await;
    match user {
        Ok(user) => {
            let posts = Post::get_all(&username, &app_state.pg).await;
            let posts = match posts {
                Ok(posts) => posts,
                Err(_) => vec![],
            };

            return Ok(UserProfile { base, user, posts });
        }
        Err(_) => return Err((axum::http::StatusCode::NOT_FOUND, NotFound { base })),
    }
}
