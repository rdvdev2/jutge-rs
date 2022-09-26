use thiserror::Error;

/// The [`Result`](std::result::Result) type produced by the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The errors produced by the crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Wraps arround errors in the [`ureq`] crate.
    #[cfg(feature="dep:ureq")]
    #[error("ureq error: {0}")]
    UreqError(#[from] ureq::Error),

    /// Indicates an impossible conversion from a `char` to a
    /// [`ProblemType`](crate::ProblemType)
    #[error("char doesn't represent a valid problem type")]
    NotAProblemType,

    /// Indicates that a [`ProblemId`](crate::ProblemId) couldn't be created
    /// because the given inputs don't constitute a valid problem id.
    #[error("invalid problem id: {0}")]
    InvalidProblemId(String),
}
