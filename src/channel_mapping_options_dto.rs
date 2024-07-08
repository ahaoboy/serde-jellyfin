use crate::name_id_pair::NameIdPair;
use crate::name_value_pair::NameValuePair;
use crate::tuner_channel_mapping::TunerChannelMapping;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelMappingOptionsDto {
    #[serde(rename = "TunerChannels", skip_serializing_if = "Option:: is_none")]
    pub tuner_channels: Option<Vec<TunerChannelMapping>>,
    #[serde(rename = "ProviderChannels", skip_serializing_if = "Option:: is_none")]
    pub provider_channels: Option<Vec<NameIdPair>>,
    #[serde(rename = "Mappings", skip_serializing_if = "Option:: is_none")]
    pub mappings: Option<Vec<NameValuePair>>,
    #[serde(rename = "ProviderName", skip_serializing_if = "Option:: is_none")]
    pub provider_name: Option<String>,
}
