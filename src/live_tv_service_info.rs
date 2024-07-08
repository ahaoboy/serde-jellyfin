use crate::LiveTvServiceStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LiveTvServiceInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "HomePageUrl", skip_serializing_if = "Option:: is_none")]
    pub home_page_url: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option:: is_none")]
    pub status: Option<LiveTvServiceStatus>,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option:: is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option:: is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "HasUpdateAvailable",
        skip_serializing_if = "Option:: is_none"
    )]
    pub has_update_available: Option<bool>,
    #[serde(rename = "IsVisible", skip_serializing_if = "Option:: is_none")]
    pub is_visible: Option<bool>,
    #[serde(rename = "Tuners", skip_serializing_if = "Option:: is_none")]
    pub tuners: Option<Vec<String>>,
}
