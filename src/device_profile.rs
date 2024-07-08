use crate::codec_profile::CodecProfile;
use crate::container_profile::ContainerProfile;
use crate::device_identification::DeviceIdentification;
use crate::direct_play_profile::DirectPlayProfile;
use crate::response_profile::ResponseProfile;
use crate::subtitle_profile::SubtitleProfile;
use crate::transcoding_profile::TranscodingProfile;
use crate::xml_attribute::XmlAttribute;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceProfile {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Identification", skip_serializing_if = "Option:: is_none")]
    pub identification: Option<DeviceIdentification>,
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option:: is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option:: is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "ManufacturerUrl", skip_serializing_if = "Option:: is_none")]
    pub manufacturer_url: Option<String>,
    #[serde(rename = "ModelName", skip_serializing_if = "Option:: is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelDescription", skip_serializing_if = "Option:: is_none")]
    pub model_description: Option<String>,
    #[serde(rename = "ModelNumber", skip_serializing_if = "Option:: is_none")]
    pub model_number: Option<String>,
    #[serde(rename = "ModelUrl", skip_serializing_if = "Option:: is_none")]
    pub model_url: Option<String>,
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option:: is_none")]
    pub serial_number: Option<String>,
    #[serde(
        rename = "EnableAlbumArtInDidl",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_album_art_in_didl: Option<bool>,
    #[serde(
        rename = "EnableSingleAlbumArtLimit",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_single_album_art_limit: Option<bool>,
    #[serde(
        rename = "EnableSingleSubtitleLimit",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_single_subtitle_limit: Option<bool>,
    #[serde(
        rename = "SupportedMediaTypes",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supported_media_types: Option<String>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "AlbumArtPn", skip_serializing_if = "Option:: is_none")]
    pub album_art_pn: Option<String>,
    #[serde(rename = "MaxAlbumArtWidth", skip_serializing_if = "Option:: is_none")]
    pub max_album_art_width: Option<f32>,
    #[serde(rename = "MaxAlbumArtHeight", skip_serializing_if = "Option:: is_none")]
    pub max_album_art_height: Option<f32>,
    #[serde(rename = "MaxIconWidth", skip_serializing_if = "Option:: is_none")]
    pub max_icon_width: Option<f32>,
    #[serde(rename = "MaxIconHeight", skip_serializing_if = "Option:: is_none")]
    pub max_icon_height: Option<f32>,
    #[serde(
        rename = "MaxStreamingBitrate",
        skip_serializing_if = "Option:: is_none"
    )]
    pub max_streaming_bitrate: Option<f32>,
    #[serde(rename = "MaxStaticBitrate", skip_serializing_if = "Option:: is_none")]
    pub max_static_bitrate: Option<f32>,
    #[serde(
        rename = "MusicStreamingTranscodingBitrate",
        skip_serializing_if = "Option:: is_none"
    )]
    pub music_streaming_transcoding_bitrate: Option<f32>,
    #[serde(
        rename = "MaxStaticMusicBitrate",
        skip_serializing_if = "Option:: is_none"
    )]
    pub max_static_music_bitrate: Option<f32>,
    #[serde(
        rename = "SonyAggregationFlags",
        skip_serializing_if = "Option:: is_none"
    )]
    pub sony_aggregation_flags: Option<String>,
    #[serde(rename = "ProtocolInfo", skip_serializing_if = "Option:: is_none")]
    pub protocol_info: Option<String>,
    #[serde(
        rename = "TimelineOffsetSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub timeline_offset_seconds: Option<f32>,
    #[serde(
        rename = "RequiresPlainVideoItems",
        skip_serializing_if = "Option:: is_none"
    )]
    pub requires_plain_video_items: Option<bool>,
    #[serde(
        rename = "RequiresPlainFolders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub requires_plain_folders: Option<bool>,
    #[serde(
        rename = "EnableMSMediaReceiverRegistrar",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_ms_media_receiver_registrar: Option<bool>,
    #[serde(
        rename = "IgnoreTranscodeByteRangeRequests",
        skip_serializing_if = "Option:: is_none"
    )]
    pub ignore_transcode_byte_range_requests: Option<bool>,
    #[serde(rename = "XmlRootAttributes", skip_serializing_if = "Option:: is_none")]
    pub xml_root_attributes: Option<Vec<XmlAttribute>>,
    #[serde(
        rename = "DirectPlayProfiles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub direct_play_profiles: Option<Vec<DirectPlayProfile>>,
    #[serde(
        rename = "TranscodingProfiles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub transcoding_profiles: Option<Vec<TranscodingProfile>>,
    #[serde(rename = "ContainerProfiles", skip_serializing_if = "Option:: is_none")]
    pub container_profiles: Option<Vec<ContainerProfile>>,
    #[serde(rename = "CodecProfiles", skip_serializing_if = "Option:: is_none")]
    pub codec_profiles: Option<Vec<CodecProfile>>,
    #[serde(rename = "ResponseProfiles", skip_serializing_if = "Option:: is_none")]
    pub response_profiles: Option<Vec<ResponseProfile>>,
    #[serde(rename = "SubtitleProfiles", skip_serializing_if = "Option:: is_none")]
    pub subtitle_profiles: Option<Vec<SubtitleProfile>>,
}
