use crate::PlaystateCommand;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaystateRequest {
    #[serde(rename = "Command", skip_serializing_if = "Option:: is_none")]
    pub command: Option<PlaystateCommand>,
    #[serde(rename = "SeekPositionTicks", skip_serializing_if = "Option:: is_none")]
    pub seek_position_ticks: Option<f32>,
    #[serde(rename = "ControllingUserId", skip_serializing_if = "Option:: is_none")]
    pub controlling_user_id: Option<String>,
}
