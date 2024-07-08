use crate::image_type::ImageType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "ImageType", skip_serializing_if = "Option:: is_none")]
    pub image_type: Option<ImageType>,
    #[serde(rename = "ImageIndex", skip_serializing_if = "Option:: is_none")]
    pub image_index: Option<f32>,
    #[serde(rename = "ImageTag", skip_serializing_if = "Option:: is_none")]
    pub image_tag: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "BlurHash", skip_serializing_if = "Option:: is_none")]
    pub blur_hash: Option<String>,
    #[serde(rename = "Height", skip_serializing_if = "Option:: is_none")]
    pub height: Option<f32>,
    #[serde(rename = "Width", skip_serializing_if = "Option:: is_none")]
    pub width: Option<f32>,
    #[serde(rename = "Size", skip_serializing_if = "Option:: is_none")]
    pub size: Option<f32>,
}
