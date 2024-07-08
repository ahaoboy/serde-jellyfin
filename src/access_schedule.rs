use crate::dynamic_day_of_week::DynamicDayOfWeek;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessSchedule {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<f32>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "DayOfWeek", skip_serializing_if = "Option:: is_none")]
    pub day_of_week: Option<DynamicDayOfWeek>,
    #[serde(rename = "StartHour", skip_serializing_if = "Option:: is_none")]
    pub start_hour: Option<f32>,
    #[serde(rename = "EndHour", skip_serializing_if = "Option:: is_none")]
    pub end_hour: Option<f32>,
}
