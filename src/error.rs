use std::fmt;
pub enum MineError {
    IllegalArgumentConfiguration,
}

impl fmt::Display for MineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MineError::IllegalArgumentConfiguration => {
                f.write_str("Invalid argument configuration. Check src/args.rs for errors")
            }
        }
    }
}
