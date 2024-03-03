use crate::database::User;
use axum::{
    async_trait,
    extract::{Extension, FromRequestParts, OriginalUri},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};

pub struct Base {
    pub route: String,
    pub logged_in: bool,
    pub user: Option<User>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Base
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let OriginalUri(uri) = parts
            .extract::<OriginalUri>()
            .await
            .map_err(|err| err.into_response())?;
        let user = parts
            .extract::<Extension<Option<User>>>()
            .await
            .map_err(|err| err.into_response())?
            .0;
        let logged_in = user.is_some();
        Ok(Base {
            route: uri.path().to_string(),
            logged_in,
            user,
        })
    }
}
