use crate::SendToUserType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationOption {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(
        rename = "DisabledMonitorUsers",
        skip_serializing_if = "Option:: is_none"
    )]
    pub disabled_monitor_users: Option<Vec<String>>,
    #[serde(rename = "SendToUsers", skip_serializing_if = "Option:: is_none")]
    pub send_to_users: Option<Vec<String>>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option:: is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "DisabledServices", skip_serializing_if = "Option:: is_none")]
    pub disabled_services: Option<Vec<String>>,
    #[serde(rename = "SendToUserMode", skip_serializing_if = "Option:: is_none")]
    pub send_to_user_mode: Option<SendToUserType>,
}
