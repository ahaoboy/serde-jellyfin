#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathSubstitution {
    #[serde(rename = "From", skip_serializing_if = "Option:: is_none")]
    pub from: Option<String>,
    #[serde(rename = "To", skip_serializing_if = "Option:: is_none")]
    pub to: Option<String>,
}
