#[cfg(feature = "errors")]
pub type RemirResult<K> = Result<K, RemirError>;
#[cfg(feature = "errors")]
pub type RemirReturnableError = RemirError;

#[cfg(not(feature = "errors"))]
pub type RemirResult<K> = Result<K, ()>;
#[cfg(not(feature = "errors"))]
pub type RemirReturnableError = ();

#[cfg(feature = "errors")]
#[derive(Debug)]
pub struct RemirError {
    pub msg: String,
}

#[cfg(feature = "errors")]
impl RemirError {
    pub fn new(str: &str) -> Self {
        Self {
            msg: str.to_string(),
        }
    }
}

#[cfg(feature = "errors")]
#[macro_export]
macro_rules! return_err {
    ($msg: literal) => {
        return Err(crate::errs::RemirError::new($msg))
    };
}

#[cfg(not(feature = "errors"))]
#[macro_export]
macro_rules! return_err {
    ($msg: literal) => {
        return Err(())
    };
}
