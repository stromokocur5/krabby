use std::sync::Arc;

use crate::{components::Base, AppState};
use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

fn format_action(route: &str) -> String {
    format!("/api/auth{}", route)
}
#[derive(Template)]
#[template(path = "routes/auth.html")]
struct Auth {
    base: Base,
    fields: Vec<(String, String, String, String)>,
}

pub async fn auth(base: Base) -> impl IntoResponse {
    let mut fields: Vec<(String, String, String, String)> = vec![];
    if base.route == "/login" {
        fields.push((
            "username".to_owned(),
            "real".to_owned(),
            "required".to_owned(),
            "".to_owned(),
        ));
        fields.push((
            "password".to_owned(),
            "********".to_owned(),
            "required".to_owned(),
            "".to_owned(),
        ));
    }
    if base.route == "/signup" {
        fields.push((
            "username".to_owned(),
            "real".to_owned(),
            "required".to_owned(),
            "".to_owned(),
        ));
        fields.push((
            "password".to_owned(),
            "********".to_owned(),
            "required".to_owned(),
            "".to_owned(),
        ));
    }
    Auth { base, fields }
}

pub fn router() -> Router<Arc<AppState>> {
    pub async fn redirect_logged(
        base: Base,
        req: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        match base.logged_in {
            true => Ok(Redirect::to("/").into_response()),
            false => Ok(next.run(req).await),
        }
    }
    Router::new()
        .route("/login", get(auth))
        .route("/signup", get(auth))
        .route_layer(middleware::from_fn(redirect_logged))
}
