use crate::session_info::SessionInfo;
use crate::user_dto::UserDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationResult {
    #[serde(rename = "User", skip_serializing_if = "Option:: is_none")]
    pub user: Option<UserDto>,
    #[serde(rename = "SessionInfo", skip_serializing_if = "Option:: is_none")]
    pub session_info: Option<SessionInfo>,
    #[serde(rename = "AccessToken", skip_serializing_if = "Option:: is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "ServerId", skip_serializing_if = "Option:: is_none")]
    pub server_id: Option<String>,
}
