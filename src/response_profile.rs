use crate::dlna_profile_type::DlnaProfileType;
use crate::profile_condition::ProfileCondition;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProfile {
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
    #[serde(rename = "AudioCodec", skip_serializing_if = "Option:: is_none")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec", skip_serializing_if = "Option:: is_none")]
    pub video_codec: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<DlnaProfileType>,
    #[serde(rename = "OrgPn", skip_serializing_if = "Option:: is_none")]
    pub org_pn: Option<String>,
    #[serde(rename = "MimeType", skip_serializing_if = "Option:: is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "Conditions", skip_serializing_if = "Option:: is_none")]
    pub conditions: Option<Vec<ProfileCondition>>,
}
