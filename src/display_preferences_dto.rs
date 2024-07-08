use crate::ScrollDirection;
use crate::SortOrder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisplayPreferencesDto {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "ViewType", skip_serializing_if = "Option:: is_none")]
    pub view_type: Option<String>,
    #[serde(rename = "SortBy", skip_serializing_if = "Option:: is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "IndexBy", skip_serializing_if = "Option:: is_none")]
    pub index_by: Option<String>,
    #[serde(rename = "RememberIndexing", skip_serializing_if = "Option:: is_none")]
    pub remember_indexing: Option<bool>,
    #[serde(
        rename = "PrimaryImageHeight",
        skip_serializing_if = "Option:: is_none"
    )]
    pub primary_image_height: Option<f32>,
    #[serde(rename = "PrimaryImageWidth", skip_serializing_if = "Option:: is_none")]
    pub primary_image_width: Option<f32>,
    #[serde(rename = "CustomPrefs", skip_serializing_if = "Option:: is_none")]
    pub custom_prefs: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ScrollDirection", skip_serializing_if = "Option:: is_none")]
    pub scroll_direction: Option<ScrollDirection>,
    #[serde(rename = "ShowBackdrop", skip_serializing_if = "Option:: is_none")]
    pub show_backdrop: Option<bool>,
    #[serde(rename = "RememberSorting", skip_serializing_if = "Option:: is_none")]
    pub remember_sorting: Option<bool>,
    #[serde(rename = "SortOrder", skip_serializing_if = "Option:: is_none")]
    pub sort_order: Option<SortOrder>,
    #[serde(rename = "ShowSidebar", skip_serializing_if = "Option:: is_none")]
    pub show_sidebar: Option<bool>,
    #[serde(rename = "Client", skip_serializing_if = "Option:: is_none")]
    pub client: Option<String>,
}
