use crate::components::Base;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "routes/404.html")]
pub struct NotFound {
    pub base: Base,
}

pub async fn not_found(base: Base) -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, NotFound { base })
}
