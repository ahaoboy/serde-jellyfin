use crate::base_item_dto::BaseItemDto;
use crate::queue_item::QueueItem;
use crate::PlayMethod;
use crate::RepeatMode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaybackProgressInfo {
    #[serde(rename = "CanSeek", skip_serializing_if = "Option:: is_none")]
    pub can_seek: Option<bool>,
    #[serde(rename = "Item", skip_serializing_if = "Option:: is_none")]
    pub item: Option<BaseItemDto>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "SessionId", skip_serializing_if = "Option:: is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option:: is_none")]
    pub media_source_id: Option<String>,
    #[serde(rename = "AudioStreamIndex", skip_serializing_if = "Option:: is_none")]
    pub audio_stream_index: Option<f32>,
    #[serde(
        rename = "SubtitleStreamIndex",
        skip_serializing_if = "Option:: is_none"
    )]
    pub subtitle_stream_index: Option<f32>,
    #[serde(rename = "IsPaused", skip_serializing_if = "Option:: is_none")]
    pub is_paused: Option<bool>,
    #[serde(rename = "IsMuted", skip_serializing_if = "Option:: is_none")]
    pub is_muted: Option<bool>,
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option:: is_none")]
    pub position_ticks: Option<f32>,
    #[serde(
        rename = "PlaybackStartTimeTicks",
        skip_serializing_if = "Option:: is_none"
    )]
    pub playback_start_time_ticks: Option<f32>,
    #[serde(rename = "VolumeLevel", skip_serializing_if = "Option:: is_none")]
    pub volume_level: Option<f32>,
    #[serde(rename = "Brightness", skip_serializing_if = "Option:: is_none")]
    pub brightness: Option<f32>,
    #[serde(rename = "AspectRatio", skip_serializing_if = "Option:: is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "PlayMethod", skip_serializing_if = "Option:: is_none")]
    pub play_method: Option<PlayMethod>,
    #[serde(rename = "LiveStreamId", skip_serializing_if = "Option:: is_none")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "PlaySessionId", skip_serializing_if = "Option:: is_none")]
    pub play_session_id: Option<String>,
    #[serde(rename = "RepeatMode", skip_serializing_if = "Option:: is_none")]
    pub repeat_mode: Option<RepeatMode>,
    #[serde(rename = "NowPlayingQueue", skip_serializing_if = "Option:: is_none")]
    pub now_playing_queue: Option<Vec<QueueItem>>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
}
