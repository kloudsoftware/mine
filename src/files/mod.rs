use crate::error;
use crate::npm;
use async_trait::async_trait;
use std::collections::HashMap;

pub const EISEN_PACKAGE: &str = "@kloudsoftware/eisen";
pub mod package_json;

#[derive(Default)]
pub struct FileMetaInfo {
    pub project_name: String,
    pub package_versions: HashMap<String, String>,
}

impl FileMetaInfo {
    pub async fn new(project_name: &str) -> Result<Self, error::MineError> {
        let package_versions = npm::get_current_version_of_packages(vec![EISEN_PACKAGE]).await?;
        Ok(FileMetaInfo {
            project_name: project_name.to_owned(),
            package_versions,
        })
    }
}

#[async_trait]
pub trait FileGen {
    async fn generate(&self, meta: &FileMetaInfo) -> Result<(), error::MineError>;
}
