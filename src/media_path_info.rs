#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaPathInfo {
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "NetworkPath", skip_serializing_if = "Option:: is_none")]
    pub network_path: Option<String>,
}
