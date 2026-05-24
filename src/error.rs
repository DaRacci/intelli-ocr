use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntelliOcrError {
    #[error("feature not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("Unsupported environment: {0}")]
    UnsupportedEnvironment(&'static str),
}
