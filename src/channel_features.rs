use crate::channel_item_sort_field::ChannelItemSortField;
use crate::channel_media_content_type::ChannelMediaContentType;
use crate::channel_media_type::ChannelMediaType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChannelFeatures {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "CanSearch", skip_serializing_if = "Option:: is_none")]
    pub can_search: Option<bool>,
    #[serde(rename = "MediaTypes", skip_serializing_if = "Option:: is_none")]
    pub media_types: Option<Vec<ChannelMediaType>>,
    #[serde(rename = "ContentTypes", skip_serializing_if = "Option:: is_none")]
    pub content_types: Option<Vec<ChannelMediaContentType>>,
    #[serde(rename = "MaxPageSize", skip_serializing_if = "Option:: is_none")]
    pub max_page_size: Option<f32>,
    #[serde(rename = "AutoRefreshLevels", skip_serializing_if = "Option:: is_none")]
    pub auto_refresh_levels: Option<f32>,
    #[serde(rename = "DefaultSortFields", skip_serializing_if = "Option:: is_none")]
    pub default_sort_fields: Option<Vec<ChannelItemSortField>>,
    #[serde(
        rename = "SupportsSortOrderToggle",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_sort_order_toggle: Option<bool>,
    #[serde(
        rename = "SupportsLatestMedia",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_latest_media: Option<bool>,
    #[serde(rename = "CanFilter", skip_serializing_if = "Option:: is_none")]
    pub can_filter: Option<bool>,
    #[serde(
        rename = "SupportsContentDownloading",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_content_downloading: Option<bool>,
}
