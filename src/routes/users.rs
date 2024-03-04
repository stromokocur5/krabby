use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{components::Base, database::User, AppState};

use super::NotFound;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/:username", get(user_profile))
}
#[derive(Template)]
#[template(path = "routes/profile.html")]
struct UserProfile {
    user: User,
    base: Base,
}
async fn user_profile(
    State(app_state): State<Arc<AppState>>,
    Path(username): Path<String>,
    base: Base,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user = User::get(format!("username='{username}'").as_str(), &app_state.pg).await;
    match user {
        Ok(user) => return Ok(UserProfile { user, base }),
        Err(_) => return Err((axum::http::StatusCode::NOT_FOUND, NotFound { base })),
    }
}
