use crate::notification_dto::NotificationDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationResultDto {
    #[serde(rename = "Notifications", skip_serializing_if = "Option:: is_none")]
    pub notifications: Option<Vec<NotificationDto>>,
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option:: is_none")]
    pub total_record_count: Option<f32>,
}
