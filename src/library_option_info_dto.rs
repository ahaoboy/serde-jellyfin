#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryOptionInfoDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "DefaultEnabled", skip_serializing_if = "Option:: is_none")]
    pub default_enabled: Option<bool>,
}
