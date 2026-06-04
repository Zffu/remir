#[cfg(feature = "errors")]
#[derive(Debug)]
pub struct RemirError {
    pub msg: String,
}

#[cfg(feature = "errors")]
pub type RemirResult<K> = Result<K, RemirError>;

#[cfg(not(feature = "errors"))]
pub type RemirResult<K> = Result<K, ()>;
