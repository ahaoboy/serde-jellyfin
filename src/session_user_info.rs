#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionUserInfo {
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName", skip_serializing_if = "Option:: is_none")]
    pub user_name: Option<String>,
}
