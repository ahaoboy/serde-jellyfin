use crate::image_type::ImageType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageOption {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<ImageType>,
    #[serde(rename = "Limit", skip_serializing_if = "Option:: is_none")]
    pub limit: Option<f32>,
    #[serde(rename = "MinWidth", skip_serializing_if = "Option:: is_none")]
    pub min_width: Option<f32>,
}
