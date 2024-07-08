#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CountryInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "DisplayName", skip_serializing_if = "Option:: is_none")]
    pub display_name: Option<String>,
    #[serde(
        rename = "TwoLetterISORegionName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub two_letter_iso_region_name: Option<String>,
    #[serde(
        rename = "ThreeLetterISORegionName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub three_letter_iso_region_name: Option<String>,
}
