#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProblemDetails {
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub status: Option<f32>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option:: is_none")]
    pub instance: Option<String>,
}
