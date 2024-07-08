use crate::device_profile::DeviceProfile;
use crate::general_command_type::GeneralCommandType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientCapabilities {
    #[serde(
        rename = "PlayableMediaTypes",
        skip_serializing_if = "Option:: is_none"
    )]
    pub playable_media_types: Option<Vec<String>>,
    #[serde(rename = "SupportedCommands", skip_serializing_if = "Option:: is_none")]
    pub supported_commands: Option<Vec<GeneralCommandType>>,
    #[serde(
        rename = "SupportsMediaControl",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_media_control: Option<bool>,
    #[serde(
        rename = "SupportsContentUploading",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_content_uploading: Option<bool>,
    #[serde(
        rename = "MessageCallbackUrl",
        skip_serializing_if = "Option:: is_none"
    )]
    pub message_callback_url: Option<String>,
    #[serde(
        rename = "SupportsPersistentIdentifier",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_persistent_identifier: Option<bool>,
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option:: is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "DeviceProfile", skip_serializing_if = "Option:: is_none")]
    pub device_profile: Option<DeviceProfile>,
    #[serde(rename = "AppStoreUrl", skip_serializing_if = "Option:: is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "IconUrl", skip_serializing_if = "Option:: is_none")]
    pub icon_url: Option<String>,
}
