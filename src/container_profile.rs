use crate::dlna_profile_type::DlnaProfileType;
use crate::profile_condition::ProfileCondition;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContainerProfile {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<DlnaProfileType>,
    #[serde(rename = "Conditions", skip_serializing_if = "Option:: is_none")]
    pub conditions: Option<Vec<ProfileCondition>>,
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
}
