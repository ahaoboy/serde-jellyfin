use crate::image_type::ImageType;
use crate::RatingType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteImageInfo {
    #[serde(rename = "ProviderName", skip_serializing_if = "Option:: is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "Url", skip_serializing_if = "Option:: is_none")]
    pub url: Option<String>,
    #[serde(rename = "ThumbnailUrl", skip_serializing_if = "Option:: is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "Height", skip_serializing_if = "Option:: is_none")]
    pub height: Option<f32>,
    #[serde(rename = "Width", skip_serializing_if = "Option:: is_none")]
    pub width: Option<f32>,
    #[serde(rename = "CommunityRating", skip_serializing_if = "Option:: is_none")]
    pub community_rating: Option<f32>,
    #[serde(rename = "VoteCount", skip_serializing_if = "Option:: is_none")]
    pub vote_count: Option<f32>,
    #[serde(rename = "Language", skip_serializing_if = "Option:: is_none")]
    pub language: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<ImageType>,
    #[serde(rename = "RatingType", skip_serializing_if = "Option:: is_none")]
    pub rating_type: Option<RatingType>,
}
