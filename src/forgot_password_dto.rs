#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForgotPasswordDto {
    #[serde(rename = "EnteredUsername")]
    pub entered_username: String,
}
