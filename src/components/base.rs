use axum::{
    async_trait,
    extract::{FromRequestParts, OriginalUri},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};

pub struct Base {
    pub route: String,
    pub logged_in: bool,
}

#[async_trait]
impl<S> FromRequestParts<S> for Base
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let OriginalUri(uri) = parts
            .extract::<OriginalUri>()
            .await
            .map_err(|err| err.into_response())?;
        Ok(Base {
            route: uri.path().to_string(),
            logged_in: false,
        })
    }
}
