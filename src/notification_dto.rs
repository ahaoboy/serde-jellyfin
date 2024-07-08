use crate::NotificationLevel;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationDto {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "Date", skip_serializing_if = "Option:: is_none")]
    pub date: Option<String>,
    #[serde(rename = "IsRead", skip_serializing_if = "Option:: is_none")]
    pub is_read: Option<bool>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option:: is_none")]
    pub description: Option<String>,
    #[serde(rename = "Url", skip_serializing_if = "Option:: is_none")]
    pub url: Option<String>,
    #[serde(rename = "Level", skip_serializing_if = "Option:: is_none")]
    pub level: Option<NotificationLevel>,
}
