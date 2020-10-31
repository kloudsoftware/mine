use std::fmt;
/// type that contains all possible errors in Mine
pub enum MineError {
    /// Illegal configuration of arguments.
    /// should never be thrown
    IllegalArgumentConfiguration,
    /// wrappper around an IO error
    IOError(std::io::Error),
    /// thrown when the anticipated project dir already exists
    ProjectDirAlreadyExists(String),
}

impl fmt::Display for MineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MineError::IllegalArgumentConfiguration => write!(
                f,
                "Invalid argument configuration. Check src/args.rs for errors"
            ),
            MineError::IOError(cause) => write!(f, "general io error '{}'", cause),
            MineError::ProjectDirAlreadyExists(dir) => {
                write!(f, "the project dir '{}' already exists", dir)
            }
        }
    }
}

impl From<std::io::Error> for MineError {
    fn from(err: std::io::Error) -> Self {
        MineError::IOError(err)
    }
}
