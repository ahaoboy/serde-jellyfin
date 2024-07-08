use crate::LogLevel;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityLogEntry {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<f32>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Overview", skip_serializing_if = "Option:: is_none")]
    pub overview: Option<String>,
    #[serde(rename = "ShortOverview", skip_serializing_if = "Option:: is_none")]
    pub short_overview: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "Date", skip_serializing_if = "Option:: is_none")]
    pub date: Option<String>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(
        rename = "UserPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub user_primary_image_tag: Option<String>,
    #[serde(rename = "Severity", skip_serializing_if = "Option:: is_none")]
    pub severity: Option<LogLevel>,
}
