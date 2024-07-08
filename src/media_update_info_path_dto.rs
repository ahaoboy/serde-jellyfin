#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaUpdateInfoPathDto {
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "UpdateType", skip_serializing_if = "Option:: is_none")]
    pub update_type: Option<String>,
}
