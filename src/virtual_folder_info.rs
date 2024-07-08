use crate::collection_type_options::CollectionTypeOptions;
use crate::library_options::LibraryOptions;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VirtualFolderInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Locations", skip_serializing_if = "Option:: is_none")]
    pub locations: Option<Vec<String>>,
    #[serde(rename = "CollectionType", skip_serializing_if = "Option:: is_none")]
    pub collection_type: Option<CollectionTypeOptions>,
    #[serde(rename = "LibraryOptions", skip_serializing_if = "Option:: is_none")]
    pub library_options: Option<LibraryOptions>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(
        rename = "PrimaryImageItemId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub primary_image_item_id: Option<String>,
    #[serde(rename = "RefreshProgress", skip_serializing_if = "Option:: is_none")]
    pub refresh_progress: Option<f32>,
    #[serde(rename = "RefreshStatus", skip_serializing_if = "Option:: is_none")]
    pub refresh_status: Option<String>,
}
