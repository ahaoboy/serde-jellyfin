use crate::PluginStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option:: is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "ConfigurationFileName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub configuration_file_name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option:: is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "CanUninstall", skip_serializing_if = "Option:: is_none")]
    pub can_uninstall: Option<bool>,
    #[serde(rename = "HasImage", skip_serializing_if = "Option:: is_none")]
    pub has_image: Option<bool>,
    #[serde(rename = "Status", skip_serializing_if = "Option:: is_none")]
    pub status: Option<PluginStatus>,
}
