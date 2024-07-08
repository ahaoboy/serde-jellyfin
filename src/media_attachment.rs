#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaAttachment {
    #[serde(rename = "Codec", skip_serializing_if = "Option:: is_none")]
    pub codec: Option<String>,
    #[serde(rename = "CodecTag", skip_serializing_if = "Option:: is_none")]
    pub codec_tag: Option<String>,
    #[serde(rename = "Comment", skip_serializing_if = "Option:: is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Index", skip_serializing_if = "Option:: is_none")]
    pub index: Option<f32>,
    #[serde(rename = "FileName", skip_serializing_if = "Option:: is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "MimeType", skip_serializing_if = "Option:: is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "DeliveryUrl", skip_serializing_if = "Option:: is_none")]
    pub delivery_url: Option<String>,
}
