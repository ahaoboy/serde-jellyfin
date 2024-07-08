#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateUserPassword {
    #[serde(rename = "CurrentPassword", skip_serializing_if = "Option:: is_none")]
    pub current_password: Option<String>,
    #[serde(rename = "CurrentPw", skip_serializing_if = "Option:: is_none")]
    pub current_pw: Option<String>,
    #[serde(rename = "NewPw", skip_serializing_if = "Option:: is_none")]
    pub new_pw: Option<String>,
    #[serde(rename = "ResetPassword", skip_serializing_if = "Option:: is_none")]
    pub reset_password: Option<bool>,
}
