#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FontFile {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Size", skip_serializing_if = "Option:: is_none")]
    pub size: Option<f32>,
    #[serde(rename = "DateCreated", skip_serializing_if = "Option:: is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateModified", skip_serializing_if = "Option:: is_none")]
    pub date_modified: Option<String>,
}
