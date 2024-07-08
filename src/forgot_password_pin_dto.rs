#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForgotPasswordPinDto {
    #[serde(rename = "Pin")]
    pub pin: String,
}
