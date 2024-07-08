#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryFiltersLegacy {
    #[serde(rename = "Genres", skip_serializing_if = "Option:: is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "Tags", skip_serializing_if = "Option:: is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "OfficialRatings", skip_serializing_if = "Option:: is_none")]
    pub official_ratings: Option<Vec<String>>,
    #[serde(rename = "Years", skip_serializing_if = "Option:: is_none")]
    pub years: Option<Vec<f32>>,
}
