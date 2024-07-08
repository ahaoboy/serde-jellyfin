#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteSubtitleInfo {
    #[serde(
        rename = "ThreeLetterISOLanguageName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub three_letter_iso_language_name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "ProviderName", skip_serializing_if = "Option:: is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Format", skip_serializing_if = "Option:: is_none")]
    pub format: Option<String>,
    #[serde(rename = "Author", skip_serializing_if = "Option:: is_none")]
    pub author: Option<String>,
    #[serde(rename = "Comment", skip_serializing_if = "Option:: is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DateCreated", skip_serializing_if = "Option:: is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "CommunityRating", skip_serializing_if = "Option:: is_none")]
    pub community_rating: Option<f32>,
    #[serde(rename = "DownloadCount", skip_serializing_if = "Option:: is_none")]
    pub download_count: Option<f32>,
    #[serde(rename = "IsHashMatch", skip_serializing_if = "Option:: is_none")]
    pub is_hash_match: Option<bool>,
}
