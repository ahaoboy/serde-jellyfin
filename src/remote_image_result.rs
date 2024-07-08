use crate::remote_image_info::RemoteImageInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteImageResult {
    #[serde(rename = "Images", skip_serializing_if = "Option:: is_none")]
    pub images: Option<Vec<RemoteImageInfo>>,
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option:: is_none")]
    pub total_record_count: Option<f32>,
    #[serde(rename = "Providers", skip_serializing_if = "Option:: is_none")]
    pub providers: Option<Vec<String>>,
}
