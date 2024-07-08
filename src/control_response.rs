#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ControlResponse {
    #[serde(rename = "Headers", skip_serializing_if = "Option:: is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Xml", skip_serializing_if = "Option:: is_none")]
    pub xml: Option<String>,
    #[serde(rename = "IsSuccessful", skip_serializing_if = "Option:: is_none")]
    pub is_successful: Option<bool>,
}
