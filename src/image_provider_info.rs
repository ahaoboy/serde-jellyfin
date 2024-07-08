use crate::image_type::ImageType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageProviderInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "SupportedImages", skip_serializing_if = "Option:: is_none")]
    pub supported_images: Option<Vec<ImageType>>,
}
