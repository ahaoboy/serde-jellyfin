use crate::architecture::Architecture;
use crate::ffmpeg_location::FFmpegLocation;
use crate::installation_info::InstallationInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "LocalAddress", skip_serializing_if = "Option:: is_none")]
    pub local_address: Option<String>,
    #[serde(rename = "ServerName", skip_serializing_if = "Option:: is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option:: is_none")]
    pub version: Option<String>,
    #[serde(rename = "ProductName", skip_serializing_if = "Option:: is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "OperatingSystem", skip_serializing_if = "Option:: is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "StartupWizardCompleted",
        skip_serializing_if = "Option:: is_none"
    )]
    pub startup_wizard_completed: Option<bool>,
    #[serde(
        rename = "OperatingSystemDisplayName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub operating_system_display_name: Option<String>,
    #[serde(rename = "PackageName", skip_serializing_if = "Option:: is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "HasPendingRestart", skip_serializing_if = "Option:: is_none")]
    pub has_pending_restart: Option<bool>,
    #[serde(rename = "IsShuttingDown", skip_serializing_if = "Option:: is_none")]
    pub is_shutting_down: Option<bool>,
    #[serde(
        rename = "SupportsLibraryMonitor",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_library_monitor: Option<bool>,
    #[serde(
        rename = "WebSocketPortNumber",
        skip_serializing_if = "Option:: is_none"
    )]
    pub web_socket_port_number: Option<f32>,
    #[serde(
        rename = "CompletedInstallations",
        skip_serializing_if = "Option:: is_none"
    )]
    pub completed_installations: Option<Vec<InstallationInfo>>,
    #[serde(rename = "CanSelfRestart", skip_serializing_if = "Option:: is_none")]
    pub can_self_restart: Option<bool>,
    #[serde(
        rename = "CanLaunchWebBrowser",
        skip_serializing_if = "Option:: is_none"
    )]
    pub can_launch_web_browser: Option<bool>,
    #[serde(rename = "ProgramDataPath", skip_serializing_if = "Option:: is_none")]
    pub program_data_path: Option<String>,
    #[serde(rename = "WebPath", skip_serializing_if = "Option:: is_none")]
    pub web_path: Option<String>,
    #[serde(rename = "ItemsByNamePath", skip_serializing_if = "Option:: is_none")]
    pub items_by_name_path: Option<String>,
    #[serde(rename = "CachePath", skip_serializing_if = "Option:: is_none")]
    pub cache_path: Option<String>,
    #[serde(rename = "LogPath", skip_serializing_if = "Option:: is_none")]
    pub log_path: Option<String>,
    #[serde(
        rename = "InternalMetadataPath",
        skip_serializing_if = "Option:: is_none"
    )]
    pub internal_metadata_path: Option<String>,
    #[serde(
        rename = "TranscodingTempPath",
        skip_serializing_if = "Option:: is_none"
    )]
    pub transcoding_temp_path: Option<String>,
    #[serde(
        rename = "HasUpdateAvailable",
        skip_serializing_if = "Option:: is_none"
    )]
    pub has_update_available: Option<bool>,
    #[serde(rename = "EncoderLocation", skip_serializing_if = "Option:: is_none")]
    pub encoder_location: Option<FFmpegLocation>,
    #[serde(
        rename = "SystemArchitecture",
        skip_serializing_if = "Option:: is_none"
    )]
    pub system_architecture: Option<Architecture>,
}
