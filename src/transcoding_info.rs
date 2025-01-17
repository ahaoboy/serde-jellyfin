use crate::hardware_encoding_type::HardwareEncodingType;
use crate::TranscodeReason;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranscodingInfo {
    #[serde(rename = "AudioCodec", skip_serializing_if = "Option:: is_none")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec", skip_serializing_if = "Option:: is_none")]
    pub video_codec: Option<String>,
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
    #[serde(rename = "IsVideoDirect", skip_serializing_if = "Option:: is_none")]
    pub is_video_direct: Option<bool>,
    #[serde(rename = "IsAudioDirect", skip_serializing_if = "Option:: is_none")]
    pub is_audio_direct: Option<bool>,
    #[serde(rename = "Bitrate", skip_serializing_if = "Option:: is_none")]
    pub bitrate: Option<f32>,
    #[serde(rename = "Framerate", skip_serializing_if = "Option:: is_none")]
    pub framerate: Option<f32>,
    #[serde(
        rename = "CompletionPercentage",
        skip_serializing_if = "Option:: is_none"
    )]
    pub completion_percentage: Option<f32>,
    #[serde(rename = "Width", skip_serializing_if = "Option:: is_none")]
    pub width: Option<f32>,
    #[serde(rename = "Height", skip_serializing_if = "Option:: is_none")]
    pub height: Option<f32>,
    #[serde(rename = "AudioChannels", skip_serializing_if = "Option:: is_none")]
    pub audio_channels: Option<f32>,
    #[serde(
        rename = "HardwareAccelerationType",
        skip_serializing_if = "Option:: is_none"
    )]
    pub hardware_acceleration_type: Option<HardwareEncodingType>,
    #[serde(rename = "TranscodeReasons", skip_serializing_if = "Option:: is_none")]
    pub transcode_reasons: Option<Vec<TranscodeReason>>,
}
