use crate::components::Base;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "routes/index.html")]
struct Index {
    base: Base,
}

pub async fn index(base: Base) -> impl IntoResponse {
    Index { base }
}
