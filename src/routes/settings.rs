use crate::components::Base;
use askama::Template;
use axum::response::{IntoResponse, Redirect, Response};

#[derive(Template)]
#[template(path = "routes/settings.html")]
struct Settings {
    base: Base,
}

pub async fn settings(base: Base) -> Response {
    if !base.logged_in {
        return Redirect::to("/").into_response();
    }
    Settings { base }.into_response()
}
