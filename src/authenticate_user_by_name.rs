#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticateUserByName {
    #[serde(rename = "Username", skip_serializing_if = "Option:: is_none")]
    pub username: Option<String>,
    #[serde(rename = "Pw", skip_serializing_if = "Option:: is_none")]
    pub pw: Option<String>,
    #[serde(rename = "Password", skip_serializing_if = "Option:: is_none")]
    pub password: Option<String>,
}
