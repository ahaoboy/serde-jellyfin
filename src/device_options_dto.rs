#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceOptionsDto {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<f32>,
    #[serde(rename = "DeviceId", skip_serializing_if = "Option:: is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "CustomName", skip_serializing_if = "Option:: is_none")]
    pub custom_name: Option<String>,
}
