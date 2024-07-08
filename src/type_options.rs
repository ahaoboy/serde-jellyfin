use crate::image_option::ImageOption;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeOptions {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "MetadataFetchers", skip_serializing_if = "Option:: is_none")]
    pub metadata_fetchers: Option<Vec<String>>,
    #[serde(
        rename = "MetadataFetcherOrder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub metadata_fetcher_order: Option<Vec<String>>,
    #[serde(rename = "ImageFetchers", skip_serializing_if = "Option:: is_none")]
    pub image_fetchers: Option<Vec<String>>,
    #[serde(rename = "ImageFetcherOrder", skip_serializing_if = "Option:: is_none")]
    pub image_fetcher_order: Option<Vec<String>>,
    #[serde(rename = "ImageOptions", skip_serializing_if = "Option:: is_none")]
    pub image_options: Option<Vec<ImageOption>>,
}
