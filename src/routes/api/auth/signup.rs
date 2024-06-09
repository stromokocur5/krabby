use std::sync::Arc;

use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    Form,
};

use crate::{
    database::{SignUpUser, User},
    AppError, AppState,
};

use super::get_auth_cookies;

pub async fn signup(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Form(user): Form<SignUpUser>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = User::create(&user, &app_state.pg).await?;
    let cookies =
        get_auth_cookies(&headers, &user_id, &user.cf_turnstile_response, &app_state).await?;

    Ok((cookies, Redirect::to("/")).into_response())
}
