use crate::SubtitleDeliveryMethod;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubtitleProfile {
    #[serde(rename = "Format", skip_serializing_if = "Option:: is_none")]
    pub format: Option<String>,
    #[serde(rename = "Method", skip_serializing_if = "Option:: is_none")]
    pub method: Option<SubtitleDeliveryMethod>,
    #[serde(rename = "DidlMode", skip_serializing_if = "Option:: is_none")]
    pub didl_mode: Option<String>,
    #[serde(rename = "Language", skip_serializing_if = "Option:: is_none")]
    pub language: Option<String>,
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
}
