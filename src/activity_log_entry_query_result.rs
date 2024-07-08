use crate::activity_log_entry::ActivityLogEntry;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityLogEntryQueryResult {
    #[serde(rename = "Items", skip_serializing_if = "Option:: is_none")]
    pub items: Option<Vec<ActivityLogEntry>>,
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option:: is_none")]
    pub total_record_count: Option<f32>,
    #[serde(rename = "StartIndex", skip_serializing_if = "Option:: is_none")]
    pub start_index: Option<f32>,
}
