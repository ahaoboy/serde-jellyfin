use crate::codec_type::CodecType;
use crate::profile_condition::ProfileCondition;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodecProfile {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<CodecType>,
    #[serde(rename = "Conditions", skip_serializing_if = "Option:: is_none")]
    pub conditions: Option<Vec<ProfileCondition>>,
    #[serde(rename = "ApplyConditions", skip_serializing_if = "Option:: is_none")]
    pub apply_conditions: Option<Vec<ProfileCondition>>,
    #[serde(rename = "Codec", skip_serializing_if = "Option:: is_none")]
    pub codec: Option<String>,
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
}
