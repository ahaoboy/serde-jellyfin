use crate::dlna_profile_type::DlnaProfileType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirectPlayProfile {
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
    #[serde(rename = "AudioCodec", skip_serializing_if = "Option:: is_none")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec", skip_serializing_if = "Option:: is_none")]
    pub video_codec: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<DlnaProfileType>,
}
