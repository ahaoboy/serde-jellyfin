#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EndPointInfo {
    #[serde(rename = "IsLocal", skip_serializing_if = "Option:: is_none")]
    pub is_local: Option<bool>,
    #[serde(rename = "IsInNetwork", skip_serializing_if = "Option:: is_none")]
    pub is_in_network: Option<bool>,
}
