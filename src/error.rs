#[derive(thiserror::Error)]
pub enum Error {
    #[error("invalid regex {0}")]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("failed to move file: {0}")]
    TempfilePersist(#[from] tempfile::PersistError),
    #[error(transparent)]
    Output(#[from] crate::output::Error),
    #[error(transparent)]
    Edit(#[from] crate::edit::Error),
    #[error(transparent)]
    Writer(#[from] crate::writer::Error),
    #[error("{0}")]
    InvalidArguments(String),
}

// pretty-print the error
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
