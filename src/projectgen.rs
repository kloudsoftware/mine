use crate::args;
use crate::error;
use crate::files::{self, FileGen};
use crate::git;
use crate::npm;
use clap::ArgMatches;
use std::fs;
use std::path;

pub async fn generate(args: ArgMatches<'_>) -> Result<(), error::MineError> {
    let project_name = args.value_of(args::ARG_PROJECT_NAME).unwrap();

    if path::Path::new(&project_name).exists() {
        return Err(error::MineError::ProjectDirAlreadyExists(
            project_name.to_string(),
        ));
    }

    fs::create_dir(project_name)?;
    git::git_init(project_name)?;

    let package_versions =
        npm::get_current_version_of_packages(vec!["@kloudsoftware/eisen"]).await?;

    let meta = files::FileMetaInfo {
        project_name: project_name.to_owned(),
        package_versions,
    };

    let package_json_gen = files::package_json::PackageJsonGenerator {};
    package_json_gen.generate(&meta).await?;
    Ok(())
}
