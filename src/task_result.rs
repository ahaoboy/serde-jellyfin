use crate::TaskCompletionStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskResult {
    #[serde(rename = "StartTimeUtc", skip_serializing_if = "Option:: is_none")]
    pub start_time_utc: Option<String>,
    #[serde(rename = "EndTimeUtc", skip_serializing_if = "Option:: is_none")]
    pub end_time_utc: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option:: is_none")]
    pub status: Option<TaskCompletionStatus>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Key", skip_serializing_if = "Option:: is_none")]
    pub key: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "ErrorMessage", skip_serializing_if = "Option:: is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LongErrorMessage", skip_serializing_if = "Option:: is_none")]
    pub long_error_message: Option<String>,
}
