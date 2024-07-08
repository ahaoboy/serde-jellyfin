use crate::version_info::VersionInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageInfo {
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub overview: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub guid: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub versions: Option<Vec<VersionInfo>>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option:: is_none")]
    pub image_url: Option<String>,
}
