use crate::file_system_entry_type::FileSystemEntryType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileSystemEntryInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<FileSystemEntryType>,
}
