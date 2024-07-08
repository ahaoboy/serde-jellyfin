use crate::device_profile::DeviceProfile;
use crate::MediaProtocol;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenLiveStreamDto {
    #[serde(rename = "OpenToken", skip_serializing_if = "Option:: is_none")]
    pub open_token: Option<String>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "PlaySessionId", skip_serializing_if = "Option:: is_none")]
    pub play_session_id: Option<String>,
    #[serde(
        rename = "MaxStreamingBitrate",
        skip_serializing_if = "Option:: is_none"
    )]
    pub max_streaming_bitrate: Option<f32>,
    #[serde(rename = "StartTimeTicks", skip_serializing_if = "Option:: is_none")]
    pub start_time_ticks: Option<f32>,
    #[serde(rename = "AudioStreamIndex", skip_serializing_if = "Option:: is_none")]
    pub audio_stream_index: Option<f32>,
    #[serde(
        rename = "SubtitleStreamIndex",
        skip_serializing_if = "Option:: is_none"
    )]
    pub subtitle_stream_index: Option<f32>,
    #[serde(rename = "MaxAudioChannels", skip_serializing_if = "Option:: is_none")]
    pub max_audio_channels: Option<f32>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "EnableDirectPlay", skip_serializing_if = "Option:: is_none")]
    pub enable_direct_play: Option<bool>,
    #[serde(
        rename = "EnableDirectStream",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_direct_stream: Option<bool>,
    #[serde(rename = "DeviceProfile", skip_serializing_if = "Option:: is_none")]
    pub device_profile: Option<DeviceProfile>,
    #[serde(
        rename = "DirectPlayProtocols",
        skip_serializing_if = "Option:: is_none"
    )]
    pub direct_play_protocols: Option<Vec<MediaProtocol>>,
}
