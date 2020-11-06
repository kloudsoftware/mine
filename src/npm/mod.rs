mod types;
use crate::error;
use reqwest;
use std::collections::HashMap;

pub async fn get_current_version_of_packages(
    packages: Vec<&str>,
) -> Result<HashMap<String, String>, error::MineError> {
    let mut map: HashMap<String, String> = HashMap::new();
    for package in packages {
        let version = get_current_version_of_package(package).await?;
        map.insert(package.to_owned().to_string(), version);
    }
    Ok(map)
}

pub async fn get_current_version_of_package(package: &str) -> Result<String, error::MineError> {
    println!("fetching package {}", package);
    let response: types::NpmResponse =
        reqwest::get(&format!("https://api.npms.io/v2/search?q={}", package))
            .await?
            .json::<types::NpmResponse>()
            .await?;

    Ok(response.results[0].package.version.clone())
}
