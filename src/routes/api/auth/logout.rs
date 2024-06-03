use axum::{
    body::Body,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::AppError;

pub async fn logout() -> impl IntoResponse {
    let mut remove_user_id = Cookie::new("user_id", "");
    remove_user_id.set_path("/");
    remove_user_id.make_removal();

    let mut remove_session_id = Cookie::new("user_id", "");
    remove_session_id.set_path("/");
    remove_session_id.make_removal();

    let cookies = CookieJar::new().add(remove_user_id).add(remove_session_id);

    (cookies, Redirect::to("/")).into_response()
}
