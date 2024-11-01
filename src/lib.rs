pub mod git;

pub use git::error::GitDiffError;
pub use git::git_diff_rs::GitDiffWrapper;

pub type Result<T> = std::result::Result<T, GitDiffError>;
