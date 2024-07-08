#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WakeOnLanInfo {
    #[serde(rename = "MacAddress", skip_serializing_if = "Option:: is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Port", skip_serializing_if = "Option:: is_none")]
    pub port: Option<f32>,
}
