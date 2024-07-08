#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerDiscoveryInfo {
    #[serde(rename = "Address", skip_serializing_if = "Option:: is_none")]
    pub address: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "EndpointAddress", skip_serializing_if = "Option:: is_none")]
    pub endpoint_address: Option<String>,
}
