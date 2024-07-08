#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpecialViewOptionDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
}
