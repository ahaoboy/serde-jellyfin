use crate::external_id_media_type::ExternalIdMediaType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExternalIdInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Key", skip_serializing_if = "Option:: is_none")]
    pub key: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<ExternalIdMediaType>,
    #[serde(rename = "UrlFormatString", skip_serializing_if = "Option:: is_none")]
    pub url_format_string: Option<String>,
}
