#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryUpdateInfo {
    #[serde(rename = "FoldersAddedTo", skip_serializing_if = "Option:: is_none")]
    pub folders_added_to: Option<Vec<String>>,
    #[serde(
        rename = "FoldersRemovedFrom",
        skip_serializing_if = "Option:: is_none"
    )]
    pub folders_removed_from: Option<Vec<String>>,
    #[serde(rename = "ItemsAdded", skip_serializing_if = "Option:: is_none")]
    pub items_added: Option<Vec<String>>,
    #[serde(rename = "ItemsRemoved", skip_serializing_if = "Option:: is_none")]
    pub items_removed: Option<Vec<String>>,
    #[serde(rename = "ItemsUpdated", skip_serializing_if = "Option:: is_none")]
    pub items_updated: Option<Vec<String>>,
    #[serde(rename = "CollectionFolders", skip_serializing_if = "Option:: is_none")]
    pub collection_folders: Option<Vec<String>>,
    #[serde(rename = "IsEmpty", skip_serializing_if = "Option:: is_none")]
    pub is_empty: Option<bool>,
}
