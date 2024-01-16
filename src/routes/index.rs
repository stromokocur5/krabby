use askama::Template;
use axum::{extract::OriginalUri, response::IntoResponse};

#[derive(Template)]
#[template(path = "routes/index.html")]
struct Index {
    route: String,
    logged_in: bool,
}

pub async fn index(uri: OriginalUri) -> impl IntoResponse {
    let uri = uri.path().to_owned();
    Index {
        route: uri,
        logged_in: false,
    }
}
