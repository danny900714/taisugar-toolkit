use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing template worksheet")]
    MissingTemplateWorksheet,

    #[error("unable to find freebie in item needs")]
    FreebieNotFound,
    
    #[error("provided item needs are empty")]
    ItemNeedsEmpty,
}
