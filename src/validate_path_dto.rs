#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidatePathDto {
    #[serde(rename = "ValidateWritable", skip_serializing_if = "Option:: is_none")]
    pub validate_writable: Option<bool>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "IsFile", skip_serializing_if = "Option:: is_none")]
    pub is_file: Option<bool>,
}
