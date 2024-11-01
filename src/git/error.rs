use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitDiffError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Invalid commit reference: {0}")]
    InvalidCommit(String),

    #[error("Repository error: {0}")]
    Repository(String),
}
