use crate::error;
use crate::files::{self, FileGen, FileMetaInfo};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::path::Path;

pub struct PackageJsonGenerator {}

/// representation of a package json file
struct PackageJson<'a> {
    /// project name
    name: &'a str,
    /// project version
    version: &'a str,
    /// description about the project
    description: &'a str,
    /// entrypoint js or ts file
    main: &'a str,
    /// script shorthands
    scripts: Vec<Script<'a>>,
    /// author of the project
    author: &'a str,
    /// the license of the project
    license: &'a str,
    /// the production dependencies
    dependencies: Vec<Dependency<'a>>,
    /// the dev dependencies
    dev_dependencies: Vec<Dependency<'a>>,
}

impl Serialize for PackageJson<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("package_json", 4)?;

        let mut dep_map = HashMap::new();
        for d in &self.dependencies {
            dep_map.insert(d.name, d.version);
        }
        let mut dev_dep_map = HashMap::new();
        for d in &self.dev_dependencies {
            dev_dep_map.insert(d.name, d.version);
        }
        let mut script_map = HashMap::new();
        for s in &self.scripts {
            script_map.insert(s.name, s.command);
        }

        s.serialize_field("name", self.name)?;
        s.serialize_field("version", self.version)?;
        s.serialize_field("description", self.description)?;
        s.serialize_field("author", self.author)?;
        s.serialize_field("main", self.main)?;
        s.serialize_field("license", self.license)?;
        s.serialize_field("dependencies", &dep_map)?;
        s.serialize_field("devDependencies", &dev_dep_map)?;
        s.serialize_field("scripts", &script_map)?;
        s.end()
    }
}

/// representation of a npm dependency
struct Dependency<'a> {
    /// the name of the dependency
    name: &'a str,
    /// the version of the dependency
    version: &'a str,
}

/// representation of a script key value  pair
/// inside a package json
struct Script<'a> {
    /// the name used to call the command
    name: &'a str,
    /// the command being executed
    command: &'a str,
}

#[async_trait]
impl FileGen for PackageJsonGenerator {
    async fn generate(&self, meta: &FileMetaInfo) -> Result<(), error::MineError> {
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
    "@kloudsoftware/eisen": "{}",
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
            meta.project_name,
            meta.package_versions.get(files::EISEN_PACKAGE).unwrap()
        );

        let fmt = &format!("{}/package.json", meta.project_name);
        let p = Path::new(fmt);
        std::fs::write(p, file)?;
        Ok(())
    }
}
