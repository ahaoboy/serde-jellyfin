use crate::PlayCommand;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayRequest {
    #[serde(rename = "ItemIds", skip_serializing_if = "Option:: is_none")]
    pub item_ids: Option<Vec<String>>,
    #[serde(
        rename = "StartPositionTicks",
        skip_serializing_if = "Option:: is_none"
    )]
    pub start_position_ticks: Option<f32>,
    #[serde(rename = "PlayCommand", skip_serializing_if = "Option:: is_none")]
    pub play_command: Option<PlayCommand>,
    #[serde(rename = "ControllingUserId", skip_serializing_if = "Option:: is_none")]
    pub controlling_user_id: Option<String>,
    #[serde(
        rename = "SubtitleStreamIndex",
        skip_serializing_if = "Option:: is_none"
    )]
    pub subtitle_stream_index: Option<f32>,
    #[serde(rename = "AudioStreamIndex", skip_serializing_if = "Option:: is_none")]
    pub audio_stream_index: Option<f32>,
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option:: is_none")]
    pub media_source_id: Option<String>,
    #[serde(rename = "StartIndex", skip_serializing_if = "Option:: is_none")]
    pub start_index: Option<f32>,
}
