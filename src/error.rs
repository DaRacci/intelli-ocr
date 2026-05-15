use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntelliOcrError {
    #[error("feature not implemented: {0}")]
    NotImplemented(&'static str),
}
