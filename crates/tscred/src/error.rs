use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] url::ParseError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    JiffError(#[from] jiff::Error),

    #[error("date parse error: {0}")]
    ParseDateError(String),
}
