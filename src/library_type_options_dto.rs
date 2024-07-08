use crate::image_option::ImageOption;
use crate::image_type::ImageType;
use crate::library_option_info_dto::LibraryOptionInfoDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryTypeOptionsDto {
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "MetadataFetchers", skip_serializing_if = "Option:: is_none")]
    pub metadata_fetchers: Option<Vec<LibraryOptionInfoDto>>,
    #[serde(rename = "ImageFetchers", skip_serializing_if = "Option:: is_none")]
    pub image_fetchers: Option<Vec<LibraryOptionInfoDto>>,
    #[serde(
        rename = "SupportedImageTypes",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supported_image_types: Option<Vec<ImageType>>,
    #[serde(
        rename = "DefaultImageOptions",
        skip_serializing_if = "Option:: is_none"
    )]
    pub default_image_options: Option<Vec<ImageOption>>,
}
