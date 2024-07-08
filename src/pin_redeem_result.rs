#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinRedeemResult {
    #[serde(rename = "Success", skip_serializing_if = "Option:: is_none")]
    pub success: Option<bool>,
    #[serde(rename = "UsersReset", skip_serializing_if = "Option:: is_none")]
    pub users_reset: Option<Vec<String>>,
}
