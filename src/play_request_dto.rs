#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayRequestDto {
    #[serde(rename = "PlayingQueue", skip_serializing_if = "Option:: is_none")]
    pub playing_queue: Option<Vec<String>>,
    #[serde(
        rename = "PlayingItemPosition",
        skip_serializing_if = "Option:: is_none"
    )]
    pub playing_item_position: Option<f32>,
    #[serde(
        rename = "StartPositionTicks",
        skip_serializing_if = "Option:: is_none"
    )]
    pub start_position_ticks: Option<f32>,
}
