use crate::components::Base;
use askama::Template;
use axum::response::IntoResponse;

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
            "email".to_owned(),
            "your@email.com".to_owned(),
            "required".to_owned(),
            "".to_owned(),
        ));
        fields.push((
            "password".to_owned(),
            "********".to_owned(),
            "required".to_owned(),
            "".to_owned(),
        ));
    } else if base.route == "/signup" {
        fields.push((
            "email".to_owned(),
            "your@email.com".to_owned(),
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
