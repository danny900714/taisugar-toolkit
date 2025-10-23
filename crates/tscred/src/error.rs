use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JiffError(#[from] jiff::Error),

    #[error("date parse error: {0}")]
    ParseDateError(String),

    #[error(transparent)]
    UreqError(#[from] ureq::Error),
}
