#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PreviousItemRequestDto {
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
}
