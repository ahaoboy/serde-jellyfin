#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TunerHostInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Url", skip_serializing_if = "Option:: is_none")]
    pub url: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "DeviceId", skip_serializing_if = "Option:: is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option:: is_none")]
    pub friendly_name: Option<String>,
    #[serde(
        rename = "ImportFavoritesOnly",
        skip_serializing_if = "Option:: is_none"
    )]
    pub import_favorites_only: Option<bool>,
    #[serde(
        rename = "AllowHWTranscoding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub allow_hw_transcoding: Option<bool>,
    #[serde(
        rename = "EnableStreamLooping",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_stream_looping: Option<bool>,
    #[serde(rename = "Source", skip_serializing_if = "Option:: is_none")]
    pub source: Option<String>,
    #[serde(rename = "TunerCount", skip_serializing_if = "Option:: is_none")]
    pub tuner_count: Option<f32>,
    #[serde(rename = "UserAgent", skip_serializing_if = "Option:: is_none")]
    pub user_agent: Option<String>,
}
