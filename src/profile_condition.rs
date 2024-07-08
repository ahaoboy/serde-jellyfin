use crate::ProfileConditionType;
use crate::ProfileConditionValue;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileCondition {
    #[serde(rename = "Condition", skip_serializing_if = "Option:: is_none")]
    pub condition: Option<ProfileConditionType>,
    #[serde(rename = "Property", skip_serializing_if = "Option:: is_none")]
    pub property: Option<ProfileConditionValue>,
    #[serde(rename = "Value", skip_serializing_if = "Option:: is_none")]
    pub value: Option<String>,
    #[serde(rename = "IsRequired", skip_serializing_if = "Option:: is_none")]
    pub is_required: Option<bool>,
}
