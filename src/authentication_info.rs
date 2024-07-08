#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<f32>,
    #[serde(rename = "AccessToken", skip_serializing_if = "Option:: is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "DeviceId", skip_serializing_if = "Option:: is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "AppName", skip_serializing_if = "Option:: is_none")]
    pub app_name: Option<String>,
    #[serde(rename = "AppVersion", skip_serializing_if = "Option:: is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "DeviceName", skip_serializing_if = "Option:: is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "IsActive", skip_serializing_if = "Option:: is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "DateCreated", skip_serializing_if = "Option:: is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateRevoked", skip_serializing_if = "Option:: is_none")]
    pub date_revoked: Option<String>,
    #[serde(rename = "DateLastActivity", skip_serializing_if = "Option:: is_none")]
    pub date_last_activity: Option<String>,
    #[serde(rename = "UserName", skip_serializing_if = "Option:: is_none")]
    pub user_name: Option<String>,
}
