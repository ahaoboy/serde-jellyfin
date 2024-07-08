#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepositoryInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Url", skip_serializing_if = "Option:: is_none")]
    pub url: Option<String>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option:: is_none")]
    pub enabled: Option<bool>,
}
