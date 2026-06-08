use std::fmt::Display;

#[cfg(feature = "errors")]
/// A common [`Result`] definition that is used across the Remir codebase
pub type RemirResult<K> = Result<K, RemirError>;

/// The error returned by a [`RemirResult`].
/// Is a seperate definitions in order to satisfy the usage of [`TryFrom`] traits
#[cfg(feature = "errors")]
pub type RemirReturnableError = RemirError;

#[cfg(not(feature = "errors"))]
pub type RemirResult<K> = Result<K, ()>;
#[cfg(not(feature = "errors"))]
pub type RemirReturnableError = ();

/// Represents a simple error. Contains an error message
#[cfg(feature = "errors")]
#[derive(Debug)]
pub struct RemirError {
    /// The message of the error
    pub msg: String,
}

#[cfg(feature = "errors")]
impl RemirError {
    /// Creates a new error with the given message
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

/// Simple macro to either return a [`RemirError`] or a void type depending on the `errors` feature.
/// If the feature is enabled, the macro will return a [`RemirError`].
/// If the feature is disabled, the macro will return a void type.
#[cfg(feature = "errors")]
#[macro_export]
macro_rules! return_err {
    ($msg: literal) => {
        return Err(crate::errs::RemirError::new($msg))
    };
}

/// Simple macro to either return a [`RemirError`] or a void type depending on the `errors` feature.
/// If the feature is enabled, the macro will return a [`RemirError`].
/// If the feature is disabled, the macro will return a void type.
#[cfg(not(feature = "errors"))]
#[macro_export]
macro_rules! return_err {
    ($msg: literal) => {
        return Err(())
    };
}

#[cfg(feature = "errors")]
impl Display for RemirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.msg.fmt(f)
    }
}
