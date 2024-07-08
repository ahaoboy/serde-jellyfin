#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationPageInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "EnableInMainMenu", skip_serializing_if = "Option:: is_none")]
    pub enable_in_main_menu: Option<bool>,
    #[serde(rename = "MenuSection", skip_serializing_if = "Option:: is_none")]
    pub menu_section: Option<String>,
    #[serde(rename = "MenuIcon", skip_serializing_if = "Option:: is_none")]
    pub menu_icon: Option<String>,
    #[serde(rename = "DisplayName", skip_serializing_if = "Option:: is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "PluginId", skip_serializing_if = "Option:: is_none")]
    pub plugin_id: Option<String>,
}
