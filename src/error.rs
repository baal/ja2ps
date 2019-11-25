use std::io;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CliError {
    Io(io::Error),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::Io(error)
    }
}

#[allow(dead_code)]
impl CliError {
    pub fn new<E>(error: E) -> CliError
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::_new(error.into())
    }
    fn _new(error: Box<dyn std::error::Error + Send + Sync>) -> CliError {
        CliError::Other(error)
    }
}
