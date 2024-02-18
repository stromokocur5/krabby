use crate::Result;

pub struct User;
// {
// pub id: String,
// pub username: String,
// pub email: String,
// pub provider: AuthProvider
// }

pub struct UserSession;

pub struct SignUpUser;
pub struct LogInUser;

impl UserSession {
    async fn verify_session(_user_id: String, _session_id: String) {}
    async fn log_in() -> Result<User> {
        Ok(User)
    }
    async fn sign_up() -> Result<User> {
        Ok(User)
    }
}
