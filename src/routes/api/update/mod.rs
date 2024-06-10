use serde::Deserialize;
use std::sync::Arc;

use axum::{
    extract::{Form, State},
    response::Response,
    response::{IntoResponse, Redirect},
    routing::post,
    Router,
};

use crate::{components::Base, database::User, AppError, AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/email", post(email))
        .route("/password", post(password))
}

#[derive(Deserialize)]
struct Email {
    email: String,
}

#[derive(Deserialize)]
struct Password {
    current_pass: String,
    new_pass: String,
}

async fn email(
    base: Base,
    State(app_state): State<Arc<AppState>>,
    Form(email): Form<Email>,
) -> Result<impl IntoResponse, AppError> {
    let user = base.user.unwrap();
    if let Some(e) = user.email {
        User::update(&user.id, "email", &email.email, &app_state.pg).await?;
        return Ok(Redirect::to("/settings"));
    }
    Err(anyhow::anyhow!("").into())
}
async fn password(
    base: Base,
    State(app_state): State<Arc<AppState>>,
    Form(password): Form<Password>,
) -> Result<impl IntoResponse, AppError> {
    let user = base.user.unwrap();
    if pwhash::bcrypt::verify(password.current_pass, &user.password_hash.unwrap()) {
        let pass_hash = pwhash::bcrypt::hash(password.new_pass.clone())
            .map_err(|_| anyhow::anyhow!("password hash error"))?;
        User::update(&user.id, "password_hash", &pass_hash, &app_state.pg).await?;
        return Ok(Redirect::to("/settings"));
    }
    Err(anyhow::anyhow!("").into())
}
