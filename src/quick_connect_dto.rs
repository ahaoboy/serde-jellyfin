#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickConnectDto {
    #[serde(rename = "Secret")]
    pub secret: String,
}
