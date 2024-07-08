use crate::base_item_dto::BaseItemDto;
use crate::queue_item::QueueItem;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaybackStopInfo {
    #[serde(rename = "Item", skip_serializing_if = "Option:: is_none")]
    pub item: Option<BaseItemDto>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "SessionId", skip_serializing_if = "Option:: is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option:: is_none")]
    pub media_source_id: Option<String>,
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option:: is_none")]
    pub position_ticks: Option<f32>,
    #[serde(rename = "LiveStreamId", skip_serializing_if = "Option:: is_none")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "PlaySessionId", skip_serializing_if = "Option:: is_none")]
    pub play_session_id: Option<String>,
    #[serde(rename = "Failed", skip_serializing_if = "Option:: is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "NextMediaType", skip_serializing_if = "Option:: is_none")]
    pub next_media_type: Option<String>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "NowPlayingQueue", skip_serializing_if = "Option:: is_none")]
    pub now_playing_queue: Option<Vec<QueueItem>>,
}
