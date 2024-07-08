use crate::media_source_info::MediaSourceInfo;
use crate::PlaybackErrorCode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaybackInfoResponse {
    #[serde(rename = "MediaSources", skip_serializing_if = "Option:: is_none")]
    pub media_sources: Option<Vec<MediaSourceInfo>>,
    #[serde(rename = "PlaySessionId", skip_serializing_if = "Option:: is_none")]
    pub play_session_id: Option<String>,
    #[serde(rename = "ErrorCode", skip_serializing_if = "Option:: is_none")]
    pub error_code: Option<PlaybackErrorCode>,
}
