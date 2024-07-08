use crate::notification_option::NotificationOption;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationOptions {
    #[serde(rename = "Options", skip_serializing_if = "Option:: is_none")]
    pub options: Option<Vec<NotificationOption>>,
}
