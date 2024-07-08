use crate::media_source_info::MediaSourceInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LiveStreamResponse {
    #[serde(rename = "MediaSource", skip_serializing_if = "Option:: is_none")]
    pub media_source: Option<MediaSourceInfo>,
}
