#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetChannelMappingDto {
    #[serde(rename = "ProviderId")]
    pub provider_id: String,
    #[serde(rename = "TunerChannelId")]
    pub tuner_channel_id: String,
    #[serde(rename = "ProviderChannelId")]
    pub provider_channel_id: String,
}
