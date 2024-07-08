#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StartupConfigurationDto {
    #[serde(rename = "UICulture", skip_serializing_if = "Option:: is_none")]
    pub ui_culture: Option<String>,
    #[serde(
        rename = "MetadataCountryCode",
        skip_serializing_if = "Option:: is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[serde(
        rename = "PreferredMetadataLanguage",
        skip_serializing_if = "Option:: is_none"
    )]
    pub preferred_metadata_language: Option<String>,
}
