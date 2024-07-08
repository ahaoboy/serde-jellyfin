#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BufferRequestDto {
    #[serde(rename = "When", skip_serializing_if = "Option:: is_none")]
    pub when: Option<String>,
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option:: is_none")]
    pub position_ticks: Option<f32>,
    #[serde(rename = "IsPlaying", skip_serializing_if = "Option:: is_none")]
    pub is_playing: Option<bool>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
}
