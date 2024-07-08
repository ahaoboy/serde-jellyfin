#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserItemDataDto {
    #[serde(rename = "Rating", skip_serializing_if = "Option:: is_none")]
    pub rating: Option<f32>,
    #[serde(rename = "PlayedPercentage", skip_serializing_if = "Option:: is_none")]
    pub played_percentage: Option<f32>,
    #[serde(rename = "UnplayedItemCount", skip_serializing_if = "Option:: is_none")]
    pub unplayed_item_count: Option<f32>,
    #[serde(
        rename = "PlaybackPositionTicks",
        skip_serializing_if = "Option:: is_none"
    )]
    pub playback_position_ticks: Option<f32>,
    #[serde(rename = "PlayCount", skip_serializing_if = "Option:: is_none")]
    pub play_count: Option<f32>,
    #[serde(rename = "IsFavorite", skip_serializing_if = "Option:: is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "Likes", skip_serializing_if = "Option:: is_none")]
    pub likes: Option<bool>,
    #[serde(rename = "LastPlayedDate", skip_serializing_if = "Option:: is_none")]
    pub last_played_date: Option<String>,
    #[serde(rename = "Played", skip_serializing_if = "Option:: is_none")]
    pub played: Option<bool>,
    #[serde(rename = "Key", skip_serializing_if = "Option:: is_none")]
    pub key: Option<String>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
}
