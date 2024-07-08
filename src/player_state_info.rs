use crate::PlayMethod;
use crate::RepeatMode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerStateInfo {
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option:: is_none")]
    pub position_ticks: Option<f32>,
    #[serde(rename = "CanSeek", skip_serializing_if = "Option:: is_none")]
    pub can_seek: Option<bool>,
    #[serde(rename = "IsPaused", skip_serializing_if = "Option:: is_none")]
    pub is_paused: Option<bool>,
    #[serde(rename = "IsMuted", skip_serializing_if = "Option:: is_none")]
    pub is_muted: Option<bool>,
    #[serde(rename = "VolumeLevel", skip_serializing_if = "Option:: is_none")]
    pub volume_level: Option<f32>,
    #[serde(rename = "AudioStreamIndex", skip_serializing_if = "Option:: is_none")]
    pub audio_stream_index: Option<f32>,
    #[serde(
        rename = "SubtitleStreamIndex",
        skip_serializing_if = "Option:: is_none"
    )]
    pub subtitle_stream_index: Option<f32>,
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option:: is_none")]
    pub media_source_id: Option<String>,
    #[serde(rename = "PlayMethod", skip_serializing_if = "Option:: is_none")]
    pub play_method: Option<PlayMethod>,
    #[serde(rename = "RepeatMode", skip_serializing_if = "Option:: is_none")]
    pub repeat_mode: Option<RepeatMode>,
    #[serde(rename = "LiveStreamId", skip_serializing_if = "Option:: is_none")]
    pub live_stream_id: Option<String>,
}
