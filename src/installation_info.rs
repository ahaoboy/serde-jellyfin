use crate::package_info::PackageInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstallationInfo {
    #[serde(rename = "Guid", skip_serializing_if = "Option:: is_none")]
    pub guid: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option:: is_none")]
    pub version: Option<String>,
    #[serde(rename = "Changelog", skip_serializing_if = "Option:: is_none")]
    pub changelog: Option<String>,
    #[serde(rename = "SourceUrl", skip_serializing_if = "Option:: is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "Checksum", skip_serializing_if = "Option:: is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "PackageInfo", skip_serializing_if = "Option:: is_none")]
    pub package_info: Option<PackageInfo>,
}
