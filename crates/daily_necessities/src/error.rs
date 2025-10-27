use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    UreqError(#[from] ureq::Error),

    #[error("CSRF token not found")]
    CSRFTokenNotFound,

    #[error("Login failed with status code {0}")]
    LoginError(u16),
}
