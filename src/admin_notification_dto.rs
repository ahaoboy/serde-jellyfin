use crate::NotificationLevel;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdminNotificationDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option:: is_none")]
    pub description: Option<String>,
    #[serde(rename = "NotificationLevel", skip_serializing_if = "Option:: is_none")]
    pub notification_level: Option<NotificationLevel>,
    #[serde(rename = "Url", skip_serializing_if = "Option:: is_none")]
    pub url: Option<String>,
}
