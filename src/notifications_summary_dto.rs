use crate::NotificationLevel;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationsSummaryDto {
    #[serde(rename = "UnreadCount", skip_serializing_if = "Option:: is_none")]
    pub unread_count: Option<f32>,
    #[serde(
        rename = "MaxUnreadNotificationLevel",
        skip_serializing_if = "Option:: is_none"
    )]
    pub max_unread_notification_level: Option<NotificationLevel>,
}
