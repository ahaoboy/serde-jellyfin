use crate::day_of_week::DayOfWeek;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskTriggerInfo {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TimeOfDayTicks", skip_serializing_if = "Option:: is_none")]
    pub time_of_day_ticks: Option<f32>,
    #[serde(rename = "IntervalTicks", skip_serializing_if = "Option:: is_none")]
    pub interval_ticks: Option<f32>,
    #[serde(rename = "DayOfWeek", skip_serializing_if = "Option:: is_none")]
    pub day_of_week: Option<DayOfWeek>,
    #[serde(rename = "MaxRuntimeTicks", skip_serializing_if = "Option:: is_none")]
    pub max_runtime_ticks: Option<f32>,
}
