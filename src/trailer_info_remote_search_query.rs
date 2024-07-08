use crate::trailer_info::TrailerInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrailerInfoRemoteSearchQuery {
    #[serde(rename = "SearchInfo", skip_serializing_if = "Option:: is_none")]
    pub search_info: Option<TrailerInfo>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(
        rename = "SearchProviderName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub search_provider_name: Option<String>,
    #[serde(
        rename = "IncludeDisabledProviders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub include_disabled_providers: Option<bool>,
}
