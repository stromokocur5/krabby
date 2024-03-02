use crate::components::Base;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "routes/login.html")]
struct LogIn {
    base: Base,
}

pub async fn login(base: Base) -> impl IntoResponse {
    LogIn { base }
}
