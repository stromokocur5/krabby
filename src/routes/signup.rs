use crate::components::Base;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "routes/signup.html")]
struct SignUp {
    base: Base,
}

pub async fn signup(base: Base) -> impl IntoResponse {
    SignUp { base }
}

mod filters {
    pub fn replace_<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace("_", " "))
    }
}
