#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParentalRating {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option:: is_none")]
    pub value: Option<f32>,
}
