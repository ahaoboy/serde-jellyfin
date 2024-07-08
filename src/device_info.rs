use crate::client_capabilities::ClientCapabilities;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "AccessToken", skip_serializing_if = "Option:: is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUserName", skip_serializing_if = "Option:: is_none")]
    pub last_user_name: Option<String>,
    #[serde(rename = "AppName", skip_serializing_if = "Option:: is_none")]
    pub app_name: Option<String>,
    #[serde(rename = "AppVersion", skip_serializing_if = "Option:: is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "LastUserId", skip_serializing_if = "Option:: is_none")]
    pub last_user_id: Option<String>,
    #[serde(rename = "DateLastActivity", skip_serializing_if = "Option:: is_none")]
    pub date_last_activity: Option<String>,
    #[serde(rename = "Capabilities", skip_serializing_if = "Option:: is_none")]
    pub capabilities: Option<ClientCapabilities>,
    #[serde(rename = "IconUrl", skip_serializing_if = "Option:: is_none")]
    pub icon_url: Option<String>,
}
