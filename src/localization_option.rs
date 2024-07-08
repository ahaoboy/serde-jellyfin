#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalizationOption {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option:: is_none")]
    pub value: Option<String>,
}
