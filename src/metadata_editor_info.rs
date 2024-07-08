use crate::country_info::CountryInfo;
use crate::culture_dto::CultureDto;
use crate::external_id_info::ExternalIdInfo;
use crate::name_value_pair::NameValuePair;
use crate::parental_rating::ParentalRating;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetadataEditorInfo {
    #[serde(
        rename = "ParentalRatingOptions",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parental_rating_options: Option<Vec<ParentalRating>>,
    #[serde(rename = "Countries", skip_serializing_if = "Option:: is_none")]
    pub countries: Option<Vec<CountryInfo>>,
    #[serde(rename = "Cultures", skip_serializing_if = "Option:: is_none")]
    pub cultures: Option<Vec<CultureDto>>,
    #[serde(rename = "ExternalIdInfos", skip_serializing_if = "Option:: is_none")]
    pub external_id_infos: Option<Vec<ExternalIdInfo>>,
    #[serde(rename = "ContentType", skip_serializing_if = "Option:: is_none")]
    pub content_type: Option<String>,
    #[serde(
        rename = "ContentTypeOptions",
        skip_serializing_if = "Option:: is_none"
    )]
    pub content_type_options: Option<Vec<NameValuePair>>,
}
