#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickConnectResult {
    #[serde(rename = "Authenticated", skip_serializing_if = "Option:: is_none")]
    pub authenticated: Option<bool>,
    #[serde(rename = "Secret", skip_serializing_if = "Option:: is_none")]
    pub secret: Option<String>,
    #[serde(rename = "Code", skip_serializing_if = "Option:: is_none")]
    pub code: Option<String>,
    #[serde(rename = "DeviceId", skip_serializing_if = "Option:: is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "DeviceName", skip_serializing_if = "Option:: is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "AppName", skip_serializing_if = "Option:: is_none")]
    pub app_name: Option<String>,
    #[serde(rename = "AppVersion", skip_serializing_if = "Option:: is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "DateAdded", skip_serializing_if = "Option:: is_none")]
    pub date_added: Option<String>,
}
