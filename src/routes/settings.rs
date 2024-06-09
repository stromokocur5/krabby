use crate::components::Base;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "routes/settings.html")]
struct Settings {
    base: Base,
}

pub async fn settings(base: Base) -> impl IntoResponse {
    Settings { base }
}
