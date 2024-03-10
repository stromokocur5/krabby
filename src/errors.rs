use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid username: {0}")]
    UserNameError(#[from] UserNameError),
    #[error("Invalid email: {0}")]
    EmailError(#[from] EmailError),
    #[error("Invalid password: {0}")]
    PasswordError(#[from] PasswordError),
    #[error("Post error: {0}")]
    PostError(#[from] PostError),
    #[error("Turnstile error: {0}")]
    TurnstileError(#[from] TurnstileError),
    #[error("Unknown error: {0}")]
    Unknown(#[source] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::UserNameError(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            AppError::EmailError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            AppError::PasswordError(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            AppError::PostError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            AppError::TurnstileError(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            _ => {
                tracing::error!("Application error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum UserNameError {
    #[error("Username cannot be blank")]
    Blank,
    #[error("Username is too short")]
    TooShort,
    #[error("Username is too long")]
    TooLong,
}

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Email cannot be blank")]
    Blank,
    #[error("Email is not valid")]
    NotValid,
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Password cannot be blank")]
    Blank,
    #[error("Password is too short")]
    TooShort,
    #[error("Password is too long")]
    TooLong,
    #[error("User does not have password")]
    NoPasswordUser,
}

#[derive(Error, Debug)]
pub enum PostError {
    #[error("Post cannot be blank")]
    Blank,
    #[error("Post is too long")]
    TooLong,
}

#[derive(Error, Debug)]
pub enum TurnstileError {
    #[error("Turnstile is not valid")]
    NotValid,
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::Unknown(error)
    }
}
