use crate::error;
use crate::files::{FileGen, FileMetaInfo};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::path::Path;

pub struct PackageJsonGenerator {}

/// representation of a package json file
#[derive(Debug, Default)]
struct PackageJson {
    /// project name
    name: String,
    /// project version
    version: String,
    /// description about the project
    description: String,
    /// entrypoint js or ts file
    main: String,
    /// script shorthands
    scripts: HashMap<String, String>,
    /// author of the project
    author: String,
    /// the license of the project
    license: String,
    /// the production dependencies
    dependencies: HashMap<String, String>,
    /// the dev dependencies
    dev_dependencies: HashMap<String, String>,
}

impl Serialize for PackageJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("package_json", 4)?;

        s.serialize_field("name", self.name.as_str())?;
        s.serialize_field("version", self.version.as_str())?;
        s.serialize_field("description", self.description.as_str())?;
        s.serialize_field("author", self.author.as_str())?;
        s.serialize_field("main", self.main.as_str())?;
        s.serialize_field("license", self.license.as_str())?;
        s.serialize_field("dependencies", &self.dependencies)?;
        s.serialize_field("devDependencies", &self.dev_dependencies)?;
        s.serialize_field("scripts", &self.scripts)?;
        s.end()
    }
}

#[async_trait]
impl FileGen for PackageJsonGenerator {
    // TODO make more fields configurable
    async fn generate(&self, meta: &FileMetaInfo) -> Result<(), error::MineError> {
        let mut package_json = PackageJson::default();
        package_json.name = meta.project_name.clone();
        package_json.dependencies = meta.package_versions.clone();
        package_json.dev_dependencies = meta.dev_package_versions.clone();
        package_json.main = "index.ts".to_string();
        package_json.version = "0.0.1".to_string();
        package_json.author = get_author();
        package_json.scripts = get_scripts();

        let fmt = &format!("{}/package.json", meta.project_name);
        let p = Path::new(fmt);

        let s = serde_json::to_string(&package_json).expect("TODO");
        std::fs::write(p, s)?;
        Ok(())
    }
}

fn get_author() -> String {
    match std::env::var("USER") {
        Ok(s) => s,
        _ => "".to_string(),
    }
}

fn get_scripts() -> HashMap<String, String> {
    let mut scripts = HashMap::new();
    scripts.insert("dev".to_string(), "npx parcel src/index.html".to_string());
    scripts.insert(
        "build".to_string(),
        "npx parcel build src/index.html".to_string(),
    );
    scripts.insert(
        "clean".to_string(),
        "rm -rf dist && rm -rf lib/".to_string(),
    );
    scripts
}
