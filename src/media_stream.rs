use crate::MediaStreamType;
use crate::SubtitleDeliveryMethod;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaStream {
    #[serde(rename = "Codec", skip_serializing_if = "Option:: is_none")]
    pub codec: Option<String>,
    #[serde(rename = "CodecTag", skip_serializing_if = "Option:: is_none")]
    pub codec_tag: Option<String>,
    #[serde(rename = "Language", skip_serializing_if = "Option:: is_none")]
    pub language: Option<String>,
    #[serde(rename = "ColorRange", skip_serializing_if = "Option:: is_none")]
    pub color_range: Option<String>,
    #[serde(rename = "ColorSpace", skip_serializing_if = "Option:: is_none")]
    pub color_space: Option<String>,
    #[serde(rename = "ColorTransfer", skip_serializing_if = "Option:: is_none")]
    pub color_transfer: Option<String>,
    #[serde(rename = "ColorPrimaries", skip_serializing_if = "Option:: is_none")]
    pub color_primaries: Option<String>,
    #[serde(rename = "DvVersionMajor", skip_serializing_if = "Option:: is_none")]
    pub dv_version_major: Option<f32>,
    #[serde(rename = "DvVersionMinor", skip_serializing_if = "Option:: is_none")]
    pub dv_version_minor: Option<f32>,
    #[serde(rename = "DvProfile", skip_serializing_if = "Option:: is_none")]
    pub dv_profile: Option<f32>,
    #[serde(rename = "DvLevel", skip_serializing_if = "Option:: is_none")]
    pub dv_level: Option<f32>,
    #[serde(rename = "RpuPresentFlag", skip_serializing_if = "Option:: is_none")]
    pub rpu_present_flag: Option<f32>,
    #[serde(rename = "ElPresentFlag", skip_serializing_if = "Option:: is_none")]
    pub el_present_flag: Option<f32>,
    #[serde(rename = "BlPresentFlag", skip_serializing_if = "Option:: is_none")]
    pub bl_present_flag: Option<f32>,
    #[serde(
        rename = "DvBlSignalCompatibilityId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub dv_bl_signal_compatibility_id: Option<f32>,
    #[serde(rename = "Comment", skip_serializing_if = "Option:: is_none")]
    pub comment: Option<String>,
    #[serde(rename = "TimeBase", skip_serializing_if = "Option:: is_none")]
    pub time_base: Option<String>,
    #[serde(rename = "CodecTimeBase", skip_serializing_if = "Option:: is_none")]
    pub codec_time_base: Option<String>,
    #[serde(rename = "Title", skip_serializing_if = "Option:: is_none")]
    pub title: Option<String>,
    #[serde(rename = "VideoRange", skip_serializing_if = "Option:: is_none")]
    pub video_range: Option<String>,
    #[serde(rename = "VideoRangeType", skip_serializing_if = "Option:: is_none")]
    pub video_range_type: Option<String>,
    #[serde(rename = "VideoDoViTitle", skip_serializing_if = "Option:: is_none")]
    pub video_do_vi_title: Option<String>,
    #[serde(
        rename = "LocalizedUndefined",
        skip_serializing_if = "Option:: is_none"
    )]
    pub localized_undefined: Option<String>,
    #[serde(rename = "LocalizedDefault", skip_serializing_if = "Option:: is_none")]
    pub localized_default: Option<String>,
    #[serde(rename = "LocalizedForced", skip_serializing_if = "Option:: is_none")]
    pub localized_forced: Option<String>,
    #[serde(rename = "LocalizedExternal", skip_serializing_if = "Option:: is_none")]
    pub localized_external: Option<String>,
    #[serde(rename = "DisplayTitle", skip_serializing_if = "Option:: is_none")]
    pub display_title: Option<String>,
    #[serde(rename = "NalLengthSize", skip_serializing_if = "Option:: is_none")]
    pub nal_length_size: Option<String>,
    #[serde(rename = "IsInterlaced", skip_serializing_if = "Option:: is_none")]
    pub is_interlaced: Option<bool>,
    #[serde(rename = "IsAVC", skip_serializing_if = "Option:: is_none")]
    pub is_avc: Option<bool>,
    #[serde(rename = "ChannelLayout", skip_serializing_if = "Option:: is_none")]
    pub channel_layout: Option<String>,
    #[serde(rename = "BitRate", skip_serializing_if = "Option:: is_none")]
    pub bit_rate: Option<f32>,
    #[serde(rename = "BitDepth", skip_serializing_if = "Option:: is_none")]
    pub bit_depth: Option<f32>,
    #[serde(rename = "RefFrames", skip_serializing_if = "Option:: is_none")]
    pub ref_frames: Option<f32>,
    #[serde(rename = "PacketLength", skip_serializing_if = "Option:: is_none")]
    pub packet_length: Option<f32>,
    #[serde(rename = "Channels", skip_serializing_if = "Option:: is_none")]
    pub channels: Option<f32>,
    #[serde(rename = "SampleRate", skip_serializing_if = "Option:: is_none")]
    pub sample_rate: Option<f32>,
    #[serde(rename = "IsDefault", skip_serializing_if = "Option:: is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "IsForced", skip_serializing_if = "Option:: is_none")]
    pub is_forced: Option<bool>,
    #[serde(rename = "Height", skip_serializing_if = "Option:: is_none")]
    pub height: Option<f32>,
    #[serde(rename = "Width", skip_serializing_if = "Option:: is_none")]
    pub width: Option<f32>,
    #[serde(rename = "AverageFrameRate", skip_serializing_if = "Option:: is_none")]
    pub average_frame_rate: Option<f32>,
    #[serde(rename = "RealFrameRate", skip_serializing_if = "Option:: is_none")]
    pub real_frame_rate: Option<f32>,
    #[serde(rename = "Profile", skip_serializing_if = "Option:: is_none")]
    pub profile: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<MediaStreamType>,
    #[serde(rename = "AspectRatio", skip_serializing_if = "Option:: is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "Index", skip_serializing_if = "Option:: is_none")]
    pub index: Option<f32>,
    #[serde(rename = "Score", skip_serializing_if = "Option:: is_none")]
    pub score: Option<f32>,
    #[serde(rename = "IsExternal", skip_serializing_if = "Option:: is_none")]
    pub is_external: Option<bool>,
    #[serde(rename = "DeliveryMethod", skip_serializing_if = "Option:: is_none")]
    pub delivery_method: Option<SubtitleDeliveryMethod>,
    #[serde(rename = "DeliveryUrl", skip_serializing_if = "Option:: is_none")]
    pub delivery_url: Option<String>,
    #[serde(rename = "IsExternalUrl", skip_serializing_if = "Option:: is_none")]
    pub is_external_url: Option<bool>,
    #[serde(
        rename = "IsTextSubtitleStream",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_text_subtitle_stream: Option<bool>,
    #[serde(
        rename = "SupportsExternalStream",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_external_stream: Option<bool>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "PixelFormat", skip_serializing_if = "Option:: is_none")]
    pub pixel_format: Option<String>,
    #[serde(rename = "Level", skip_serializing_if = "Option:: is_none")]
    pub level: Option<f32>,
    #[serde(rename = "IsAnamorphic", skip_serializing_if = "Option:: is_none")]
    pub is_anamorphic: Option<bool>,
}
