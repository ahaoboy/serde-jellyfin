#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageByNameInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Theme", skip_serializing_if = "Option:: is_none")]
    pub theme: Option<String>,
    #[serde(rename = "Context", skip_serializing_if = "Option:: is_none")]
    pub context: Option<String>,
    #[serde(rename = "FileLength", skip_serializing_if = "Option:: is_none")]
    pub file_length: Option<f32>,
    #[serde(rename = "Format", skip_serializing_if = "Option:: is_none")]
    pub format: Option<String>,
}
