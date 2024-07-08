#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CollectionCreationResult {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
}
