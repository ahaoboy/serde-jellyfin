#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IgnoreWaitRequestDto {
    #[serde(rename = "IgnoreWait", skip_serializing_if = "Option:: is_none")]
    pub ignore_wait: Option<bool>,
}
