use crate::user_configuration::UserConfiguration;
use crate::user_policy::UserPolicy;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "ServerId", skip_serializing_if = "Option:: is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "ServerName", skip_serializing_if = "Option:: is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option:: is_none")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "HasPassword", skip_serializing_if = "Option:: is_none")]
    pub has_password: Option<bool>,
    #[serde(
        rename = "HasConfiguredPassword",
        skip_serializing_if = "Option:: is_none"
    )]
    pub has_configured_password: Option<bool>,
    #[serde(
        rename = "HasConfiguredEasyPassword",
        skip_serializing_if = "Option:: is_none"
    )]
    pub has_configured_easy_password: Option<bool>,
    #[serde(rename = "EnableAutoLogin", skip_serializing_if = "Option:: is_none")]
    pub enable_auto_login: Option<bool>,
    #[serde(rename = "LastLoginDate", skip_serializing_if = "Option:: is_none")]
    pub last_login_date: Option<String>,
    #[serde(rename = "LastActivityDate", skip_serializing_if = "Option:: is_none")]
    pub last_activity_date: Option<String>,
    #[serde(rename = "Configuration", skip_serializing_if = "Option:: is_none")]
    pub configuration: Option<UserConfiguration>,
    #[serde(rename = "Policy", skip_serializing_if = "Option:: is_none")]
    pub policy: Option<UserPolicy>,
    #[serde(
        rename = "PrimaryImageAspectRatio",
        skip_serializing_if = "Option:: is_none"
    )]
    pub primary_image_aspect_ratio: Option<f32>,
}
