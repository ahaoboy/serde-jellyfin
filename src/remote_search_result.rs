#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteSearchResult {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option:: is_none")]
    pub provider_ids: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option:: is_none")]
    pub production_year: Option<f32>,
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option:: is_none")]
    pub index_number: Option<f32>,
    #[serde(rename = "IndexNumberEnd", skip_serializing_if = "Option:: is_none")]
    pub index_number_end: Option<f32>,
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option:: is_none")]
    pub parent_index_number: Option<f32>,
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option:: is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename = "ImageUrl", skip_serializing_if = "Option:: is_none")]
    pub image_url: Option<String>,
    #[serde(
        rename = "SearchProviderName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub search_provider_name: Option<String>,
    #[serde(rename = "Overview", skip_serializing_if = "Option:: is_none")]
    pub overview: Option<String>,
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option:: is_none")]
    pub album_artist: Option<Box<RemoteSearchResult>>,
    #[serde(rename = "Artists", skip_serializing_if = "Option:: is_none")]
    pub artists: Option<Vec<Box<RemoteSearchResult>>>,
}
