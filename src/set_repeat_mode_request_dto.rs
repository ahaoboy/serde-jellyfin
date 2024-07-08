use crate::group_repeat_mode::GroupRepeatMode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetRepeatModeRequestDto {
    #[serde(rename = "Mode", skip_serializing_if = "Option:: is_none")]
    pub mode: Option<GroupRepeatMode>,
}
