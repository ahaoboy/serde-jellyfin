use crate::name_guid_pair::NameGuidPair;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryFilters {
    #[serde(rename = "Genres", skip_serializing_if = "Option:: is_none")]
    pub genres: Option<Vec<NameGuidPair>>,
    #[serde(rename = "Tags", skip_serializing_if = "Option:: is_none")]
    pub tags: Option<Vec<String>>,
}
