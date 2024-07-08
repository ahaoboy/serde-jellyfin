#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimerEventInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "ProgramId", skip_serializing_if = "Option:: is_none")]
    pub program_id: Option<String>,
}
