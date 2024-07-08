use crate::SendCommandType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendCommand {
    #[serde(rename = "GroupId", skip_serializing_if = "Option:: is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "When", skip_serializing_if = "Option:: is_none")]
    pub when: Option<String>,
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option:: is_none")]
    pub position_ticks: Option<f32>,
    #[serde(rename = "Command", skip_serializing_if = "Option:: is_none")]
    pub command: Option<SendCommandType>,
    #[serde(rename = "EmittedAt", skip_serializing_if = "Option:: is_none")]
    pub emitted_at: Option<String>,
}
