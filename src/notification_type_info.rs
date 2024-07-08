#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationTypeInfo {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option:: is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Category", skip_serializing_if = "Option:: is_none")]
    pub category: Option<String>,
    #[serde(
        rename = "IsBasedOnUserEvent",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_based_on_user_event: Option<bool>,
}
