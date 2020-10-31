use crate::args;
use crate::error;
use crate::git;
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
    generate_package_json(&project_name)?;
    Ok(())
}

fn generate_package_json(project_name: &str) -> Result<(), error::MineError> {
    let file = &format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "An eisen Project",
  "main": "index.ts",
  "scripts": {{
    "dev": "npx parcel src/index.html",
    "build": "npx parcel build src/index.html",
    "clean": "rm -rf dist/ && rm -rf lib/" }},
  "author": "",
  "license": "ISC",
    "dependencies": {{
    "@kloudsoftware/eisen": "^2.1.1",
    "postcss": "^7.0.18",
    "tailwindcss": "^1.1.2"
  }},
  "devDependencies": {{
    "parcel-bundler": "^1.12.4",
    "parcel-plugin-purgecss": "^2.1.2",
    "parcel-plugin-static-files-copy": "^2.0.0",
    "sass": "^1.18.0",
    "typescript": "^3.4.2"
  }}
 }}"#,
        project_name
    );

    let fmt = &format!("{}/package.json", project_name);
    let p = path::Path::new(fmt);
    std::fs::write(p, file)?;
    Ok(())
}
