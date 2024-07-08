#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientLogDocumentResponseDto {
    #[serde(rename = "FileName", skip_serializing_if = "Option:: is_none")]
    pub file_name: Option<String>,
}
