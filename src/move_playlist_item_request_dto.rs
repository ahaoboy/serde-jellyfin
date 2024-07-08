#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MovePlaylistItemRequestDto {
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "NewIndex", skip_serializing_if = "Option:: is_none")]
    pub new_index: Option<f32>,
}
