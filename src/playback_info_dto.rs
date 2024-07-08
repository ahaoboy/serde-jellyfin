use crate::device_profile::DeviceProfile;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaybackInfoDto {
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
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
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option:: is_none")]
    pub media_source_id: Option<String>,
    #[serde(rename = "LiveStreamId", skip_serializing_if = "Option:: is_none")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "DeviceProfile", skip_serializing_if = "Option:: is_none")]
    pub device_profile: Option<DeviceProfile>,
    #[serde(rename = "EnableDirectPlay", skip_serializing_if = "Option:: is_none")]
    pub enable_direct_play: Option<bool>,
    #[serde(
        rename = "EnableDirectStream",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_direct_stream: Option<bool>,
    #[serde(rename = "EnableTranscoding", skip_serializing_if = "Option:: is_none")]
    pub enable_transcoding: Option<bool>,
    #[serde(
        rename = "AllowVideoStreamCopy",
        skip_serializing_if = "Option:: is_none"
    )]
    pub allow_video_stream_copy: Option<bool>,
    #[serde(
        rename = "AllowAudioStreamCopy",
        skip_serializing_if = "Option:: is_none"
    )]
    pub allow_audio_stream_copy: Option<bool>,
    #[serde(
        rename = "AutoOpenLiveStream",
        skip_serializing_if = "Option:: is_none"
    )]
    pub auto_open_live_stream: Option<bool>,
}
