use crate::task_result::TaskResult;
use crate::task_trigger_info::TaskTriggerInfo;
use crate::TaskState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option:: is_none")]
    pub state: Option<TaskState>,
    #[serde(
        rename = "CurrentProgressPercentage",
        skip_serializing_if = "Option:: is_none"
    )]
    pub current_progress_percentage: Option<f32>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "LastExecutionResult",
        skip_serializing_if = "Option:: is_none"
    )]
    pub last_execution_result: Option<TaskResult>,
    #[serde(rename = "Triggers", skip_serializing_if = "Option:: is_none")]
    pub triggers: Option<Vec<TaskTriggerInfo>>,
    #[serde(rename = "Description", skip_serializing_if = "Option:: is_none")]
    pub description: Option<String>,
    #[serde(rename = "Category", skip_serializing_if = "Option:: is_none")]
    pub category: Option<String>,
    #[serde(rename = "IsHidden", skip_serializing_if = "Option:: is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "Key", skip_serializing_if = "Option:: is_none")]
    pub key: Option<String>,
}
