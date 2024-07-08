#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateUserByName {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Password", skip_serializing_if = "Option:: is_none")]
    pub password: Option<String>,
}
