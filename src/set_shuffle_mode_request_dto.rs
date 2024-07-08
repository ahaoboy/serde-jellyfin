use crate::group_shuffle_mode::GroupShuffleMode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetShuffleModeRequestDto {
    #[serde(rename = "Mode", skip_serializing_if = "Option:: is_none")]
    pub mode: Option<GroupShuffleMode>,
}
