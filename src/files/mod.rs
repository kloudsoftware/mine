use crate::error;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod package_json;

#[derive(Default)]
pub struct FileMetaInfo {
    pub project_name: String,
    pub package_versions: HashMap<String, String>,
    pub dev_package_versions: HashMap<String, String>,
}

impl FileMetaInfo {
    pub fn new(
        project_name: &str,
        package_versions: HashMap<String, String>,
        dev_package_versions: HashMap<String, String>,
    ) -> Self {
        FileMetaInfo {
            project_name: project_name.to_owned(),
            package_versions,
            dev_package_versions,
        }
    }
}

#[async_trait]
pub trait FileGen {
    async fn generate(&self, meta: &FileMetaInfo) -> Result<(), error::MineError>;
}
