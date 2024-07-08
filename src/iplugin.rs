#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IPlugin {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option:: is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option:: is_none")]
    pub version: Option<String>,
    #[serde(rename = "AssemblyFilePath", skip_serializing_if = "Option:: is_none")]
    pub assembly_file_path: Option<String>,
    #[serde(rename = "CanUninstall", skip_serializing_if = "Option:: is_none")]
    pub can_uninstall: Option<bool>,
    #[serde(rename = "DataFolderPath", skip_serializing_if = "Option:: is_none")]
    pub data_folder_path: Option<String>,
}
