use crate::library_option_info_dto::LibraryOptionInfoDto;
use crate::library_type_options_dto::LibraryTypeOptionsDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryOptionsResultDto {
    #[serde(rename = "MetadataSavers", skip_serializing_if = "Option:: is_none")]
    pub metadata_savers: Option<Vec<LibraryOptionInfoDto>>,
    #[serde(rename = "MetadataReaders", skip_serializing_if = "Option:: is_none")]
    pub metadata_readers: Option<Vec<LibraryOptionInfoDto>>,
    #[serde(rename = "SubtitleFetchers", skip_serializing_if = "Option:: is_none")]
    pub subtitle_fetchers: Option<Vec<LibraryOptionInfoDto>>,
    #[serde(rename = "TypeOptions", skip_serializing_if = "Option:: is_none")]
    pub type_options: Option<Vec<LibraryTypeOptionsDto>>,
}
