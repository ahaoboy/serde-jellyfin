#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionInfo {
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub version: Option<String>,
    #[serde(rename = "VersionNumber", skip_serializing_if = "Option:: is_none")]
    pub version_number: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub changelog: Option<String>,
    #[serde(rename = "targetAbi", skip_serializing_if = "Option:: is_none")]
    pub target_abi: Option<String>,
    #[serde(rename = "sourceUrl", skip_serializing_if = "Option:: is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub checksum: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "repositoryName", skip_serializing_if = "Option:: is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "repositoryUrl", skip_serializing_if = "Option:: is_none")]
    pub repository_url: Option<String>,
}
