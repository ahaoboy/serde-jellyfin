use crate::group_queue_mode::GroupQueueMode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueueRequestDto {
    #[serde(rename = "ItemIds", skip_serializing_if = "Option:: is_none")]
    pub item_ids: Option<Vec<String>>,
    #[serde(rename = "Mode", skip_serializing_if = "Option:: is_none")]
    pub mode: Option<GroupQueueMode>,
}
