use crate::search_hint::SearchHint;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchHintResult {
    #[serde(rename = "SearchHints", skip_serializing_if = "Option:: is_none")]
    pub search_hints: Option<Vec<SearchHint>>,
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option:: is_none")]
    pub total_record_count: Option<f32>,
}
