use crate::timer_info_dto::TimerInfoDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimerInfoDtoQueryResult {
    #[serde(rename = "Items", skip_serializing_if = "Option:: is_none")]
    pub items: Option<Vec<TimerInfoDto>>,
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option:: is_none")]
    pub total_record_count: Option<f32>,
    #[serde(rename = "StartIndex", skip_serializing_if = "Option:: is_none")]
    pub start_index: Option<f32>,
}
