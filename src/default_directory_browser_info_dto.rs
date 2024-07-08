#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DefaultDirectoryBrowserInfoDto {
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
}
