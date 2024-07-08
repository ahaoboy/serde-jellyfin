#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TunerChannelMapping {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "ProviderChannelName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub provider_channel_name: Option<String>,
    #[serde(rename = "ProviderChannelId", skip_serializing_if = "Option:: is_none")]
    pub provider_channel_id: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
}
