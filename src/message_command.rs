#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageCommand {
    #[serde(rename = "Header", skip_serializing_if = "Option:: is_none")]
    pub header: Option<String>,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "TimeoutMs", skip_serializing_if = "Option:: is_none")]
    pub timeout_ms: Option<f32>,
}
