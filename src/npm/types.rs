#[derive(Debug, Serialize, Deserialize)]
pub struct NpmResponse {
    pub total: i64,
    pub results: Vec<Result>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub package: Package,
    pub score: Score,
    #[serde(rename = "searchScore")]
    pub search_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub scope: String,
    pub version: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub date: String,
    pub links: Links,
    pub author: Author,
    pub publisher: Publisher,
    pub maintainers: Vec<Publisher>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub npm: String,
    pub homepage: String,
    pub repository: String,
    pub bugs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    #[serde(rename = "final")]
    pub score_final: f64,
    pub detail: Detail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub quality: f64,
    pub popularity: f64,
    pub maintenance: f64,
}
