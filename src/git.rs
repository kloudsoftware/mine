use crate::error;
use std::path::Path;
use std::process::Command;

pub fn git_init<P: AsRef<Path>>(path: P) -> Result<(), error::MineError> {
    Command::new("git").arg("init").current_dir(path).output()?;
    Ok(())
}
