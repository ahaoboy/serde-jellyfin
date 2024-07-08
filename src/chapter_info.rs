#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChapterInfo {
    #[serde(
        rename = "StartPositionTicks",
        skip_serializing_if = "Option:: is_none"
    )]
    pub start_position_ticks: Option<f32>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "ImagePath", skip_serializing_if = "Option:: is_none")]
    pub image_path: Option<String>,
    #[serde(rename = "ImageDateModified", skip_serializing_if = "Option:: is_none")]
    pub image_date_modified: Option<String>,
    #[serde(rename = "ImageTag", skip_serializing_if = "Option:: is_none")]
    pub image_tag: Option<String>,
}
