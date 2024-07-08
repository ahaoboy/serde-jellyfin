use crate::library_options::LibraryOptions;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddVirtualFolderDto {
    #[serde(rename = "LibraryOptions", skip_serializing_if = "Option:: is_none")]
    pub library_options: Option<LibraryOptions>,
}
