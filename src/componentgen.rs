use crate::error;
use clap::ArgMatches;

pub async fn generate(_args: ArgMatches<'_>) -> Result<(), error::MineError> {
    Ok(())
}
