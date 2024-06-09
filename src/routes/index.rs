use std::sync::Arc;

use crate::{
    components::Base,
    database::{Post, UserPost},
    AppError, AppState,
};
use askama::Template;
use axum::{extract::State, response::IntoResponse};

#[derive(Template)]
#[template(path = "routes/index.html")]
struct Index {
    base: Base,
    feed: Vec<UserPost>,
}

pub async fn index(
    State(app_state): State<Arc<AppState>>,
    base: Base,
) -> Result<impl IntoResponse, AppError> {
    let feed = Post::get_feed(&app_state.pg).await?;
    Ok(Index { base, feed })
}
