#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreatePlaylistDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Ids", skip_serializing_if = "Option:: is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "MediaType", skip_serializing_if = "Option:: is_none")]
    pub media_type: Option<String>,
}
