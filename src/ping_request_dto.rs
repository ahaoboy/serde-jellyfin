#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PingRequestDto {
    #[serde(rename = "Ping", skip_serializing_if = "Option:: is_none")]
    pub ping: Option<f32>,
}
