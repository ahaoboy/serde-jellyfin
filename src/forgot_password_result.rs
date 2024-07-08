use crate::forgot_password_action::ForgotPasswordAction;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForgotPasswordResult {
    #[serde(rename = "Action", skip_serializing_if = "Option:: is_none")]
    pub action: Option<ForgotPasswordAction>,
    #[serde(rename = "PinFile", skip_serializing_if = "Option:: is_none")]
    pub pin_file: Option<String>,
    #[serde(rename = "PinExpirationDate", skip_serializing_if = "Option:: is_none")]
    pub pin_expiration_date: Option<String>,
}
