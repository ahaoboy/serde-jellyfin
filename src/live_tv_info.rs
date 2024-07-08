use crate::live_tv_service_info::LiveTvServiceInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LiveTvInfo {
    #[serde(rename = "Services", skip_serializing_if = "Option:: is_none")]
    pub services: Option<Vec<LiveTvServiceInfo>>,
    #[serde(rename = "IsEnabled", skip_serializing_if = "Option:: is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "EnabledUsers", skip_serializing_if = "Option:: is_none")]
    pub enabled_users: Option<Vec<String>>,
}
