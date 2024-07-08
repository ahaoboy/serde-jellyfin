use crate::header_match_type::HeaderMatchType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HttpHeaderInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option:: is_none")]
    pub value: Option<String>,
    #[serde(rename = "Match", skip_serializing_if = "Option:: is_none")]
    pub r#match: Option<HeaderMatchType>,
}
