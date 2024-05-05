use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::post,
    Form, Router,
};

use crate::{
    components::Base,
    database::{DeletePost, NewPost, Post},
    AppError, AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", post(create))
        .route("/delete", post(delete))
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    base: Base,
    Form(post): Form<NewPost>,
) -> Result<impl IntoResponse, AppError> {
    Post::create(&base.user.unwrap().id, &post, &app_state.pg).await?;
    Ok(Redirect::to("/"))
}
async fn delete(
    State(app_state): State<Arc<AppState>>,
    base: Base,
    Json(post): Json<DeletePost>,
) -> Result<impl IntoResponse, AppError> {
    Post::delete(&base.user.unwrap().id, &post.id, &app_state.pg).await?;
    Ok(StatusCode::OK)
}
