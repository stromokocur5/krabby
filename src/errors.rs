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
    #[error("User error: {0}")]
    UserError(#[from] UserError),
    #[error("Post error: {0}")]
    PostError(#[from] PostError),
    #[error("Turnstile error: {0}")]
    TurnstileError(#[from] TurnstileError),
    #[error("Redis error: {0}")]
    RedisError(#[from] deadpool_redis::redis::RedisError),
    #[error("Redis pool error: {0}")]
    RedisPoolError(#[from] deadpool::managed::PoolError<deadpool_redis::redis::RedisError>),
    // #[error("Database error: {0}")]
    // DatabaseError(#[from] DatabaseError),
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
            AppError::UserError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
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
    #[error("Password does not match")]
    DoesNotMatch,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User does not have a password")]
    NoPasswordUser,
    #[error("User does not exist")]
    DoesNotExist,
    #[error("Session id is not valid")]
    SessionIdNotValid,
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
