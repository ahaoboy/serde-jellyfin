#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeekRequestDto {
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option:: is_none")]
    pub position_ticks: Option<f32>,
}
