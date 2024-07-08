use crate::base_item_dto::BaseItemDto;
use crate::RecommendationType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecommendationDto {
    #[serde(rename = "Items", skip_serializing_if = "Option:: is_none")]
    pub items: Option<Vec<BaseItemDto>>,
    #[serde(
        rename = "RecommendationType",
        skip_serializing_if = "Option:: is_none"
    )]
    pub recommendation_type: Option<RecommendationType>,
    #[serde(rename = "BaselineItemName", skip_serializing_if = "Option:: is_none")]
    pub baseline_item_name: Option<String>,
    #[serde(rename = "CategoryId", skip_serializing_if = "Option:: is_none")]
    pub category_id: Option<String>,
}
