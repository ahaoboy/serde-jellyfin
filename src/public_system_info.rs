#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PublicSystemInfo {
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
}
