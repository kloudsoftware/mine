use crate::error;
use crate::files::{self, FileGen, FileMetaInfo};
use std::path::Path;

pub struct PackageJsonGenerator {}

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
