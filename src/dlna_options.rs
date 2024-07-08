#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DlnaOptions {
    #[serde(rename = "EnablePlayTo", skip_serializing_if = "Option:: is_none")]
    pub enable_play_to: Option<bool>,
    #[serde(rename = "EnableServer", skip_serializing_if = "Option:: is_none")]
    pub enable_server: Option<bool>,
    #[serde(rename = "EnableDebugLog", skip_serializing_if = "Option:: is_none")]
    pub enable_debug_log: Option<bool>,
    #[serde(
        rename = "EnablePlayToTracing",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_play_to_tracing: Option<bool>,
    #[serde(
        rename = "ClientDiscoveryIntervalSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub client_discovery_interval_seconds: Option<f32>,
    #[serde(
        rename = "AliveMessageIntervalSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub alive_message_interval_seconds: Option<f32>,
    #[serde(
        rename = "BlastAliveMessageIntervalSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub blast_alive_message_interval_seconds: Option<f32>,
    #[serde(rename = "DefaultUserId", skip_serializing_if = "Option:: is_none")]
    pub default_user_id: Option<String>,
    #[serde(
        rename = "AutoCreatePlayToProfiles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub auto_create_play_to_profiles: Option<bool>,
    #[serde(
        rename = "BlastAliveMessages",
        skip_serializing_if = "Option:: is_none"
    )]
    pub blast_alive_messages: Option<bool>,
    #[serde(
        rename = "SendOnlyMatchedHost",
        skip_serializing_if = "Option:: is_none"
    )]
    pub send_only_matched_host: Option<bool>,
}
