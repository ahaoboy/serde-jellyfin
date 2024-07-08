#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveFromPlaylistRequestDto {
    #[serde(rename = "PlaylistItemIds", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_ids: Option<Vec<String>>,
    #[serde(rename = "ClearPlaylist", skip_serializing_if = "Option:: is_none")]
    pub clear_playlist: Option<bool>,
    #[serde(rename = "ClearPlayingItem", skip_serializing_if = "Option:: is_none")]
    pub clear_playing_item: Option<bool>,
}
