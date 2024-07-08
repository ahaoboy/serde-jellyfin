#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GuideInfo {
    #[serde(rename = "StartDate", skip_serializing_if = "Option:: is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option:: is_none")]
    pub end_date: Option<String>,
}
