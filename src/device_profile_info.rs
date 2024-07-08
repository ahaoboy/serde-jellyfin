use crate::device_profile_type::DeviceProfileType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceProfileInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<DeviceProfileType>,
}
