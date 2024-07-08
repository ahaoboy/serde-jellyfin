#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "OriginalTitle", skip_serializing_if = "Option:: is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "MetadataLanguage", skip_serializing_if = "Option:: is_none")]
    pub metadata_language: Option<String>,
    #[serde(
        rename = "MetadataCountryCode",
        skip_serializing_if = "Option:: is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option:: is_none")]
    pub provider_ids: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Year", skip_serializing_if = "Option:: is_none")]
    pub year: Option<f32>,
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option:: is_none")]
    pub index_number: Option<f32>,
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option:: is_none")]
    pub parent_index_number: Option<f32>,
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option:: is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename = "IsAutomated", skip_serializing_if = "Option:: is_none")]
    pub is_automated: Option<bool>,
    #[serde(rename = "SeriesName", skip_serializing_if = "Option:: is_none")]
    pub series_name: Option<String>,
}
