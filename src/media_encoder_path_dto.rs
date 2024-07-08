#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaEncoderPathDto {
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "PathType", skip_serializing_if = "Option:: is_none")]
    pub path_type: Option<String>,
}
