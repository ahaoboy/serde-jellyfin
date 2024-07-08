#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CultureDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "DisplayName", skip_serializing_if = "Option:: is_none")]
    pub display_name: Option<String>,
    #[serde(
        rename = "TwoLetterISOLanguageName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub two_letter_iso_language_name: Option<String>,
    #[serde(
        rename = "ThreeLetterISOLanguageName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub three_letter_iso_language_name: Option<String>,
    #[serde(
        rename = "ThreeLetterISOLanguageNames",
        skip_serializing_if = "Option:: is_none"
    )]
    pub three_letter_iso_language_names: Option<Vec<String>>,
}
