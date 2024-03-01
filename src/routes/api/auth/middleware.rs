use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::{
    database::user::{BaseUser, User},
    AppState,
};

pub async fn auth(
    State(app_state): State<Arc<AppState>>,
    cookies: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id_cookie = cookies.get("user_id");
    let session_id_cookie = cookies.get("session_id");
    let extensions = req.extensions_mut();
    if let (Some(user_id_cookie), Some(session_id_cookie)) = (user_id_cookie, session_id_cookie) {
        let user_id = user_id_cookie.value();
        let session_id = session_id_cookie.value();
        let logged_in = User::verify_session(user_id, session_id, &app_state.redis)
            .await
            .is_ok();
        match logged_in {
            true => {
                let user = User::get_base(&user_id, &app_state.pg).await;
                if let Ok(user) = user {
                    extensions.insert::<Option<BaseUser>>(Some(user));
                }
            }
            false => {
                extensions.insert::<Option<BaseUser>>(None);
            }
        };
    } else {
        extensions.insert::<Option<BaseUser>>(None);
    };
    Ok(next.run(req).await)
}
