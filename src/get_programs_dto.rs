use crate::image_type::ImageType;
use crate::ItemFields;
use crate::SortOrder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetProgramsDto {
    #[serde(rename = "ChannelIds", skip_serializing_if = "Option:: is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "MinStartDate", skip_serializing_if = "Option:: is_none")]
    pub min_start_date: Option<String>,
    #[serde(rename = "HasAired", skip_serializing_if = "Option:: is_none")]
    pub has_aired: Option<bool>,
    #[serde(rename = "IsAiring", skip_serializing_if = "Option:: is_none")]
    pub is_airing: Option<bool>,
    #[serde(rename = "MaxStartDate", skip_serializing_if = "Option:: is_none")]
    pub max_start_date: Option<String>,
    #[serde(rename = "MinEndDate", skip_serializing_if = "Option:: is_none")]
    pub min_end_date: Option<String>,
    #[serde(rename = "MaxEndDate", skip_serializing_if = "Option:: is_none")]
    pub max_end_date: Option<String>,
    #[serde(rename = "IsMovie", skip_serializing_if = "Option:: is_none")]
    pub is_movie: Option<bool>,
    #[serde(rename = "IsSeries", skip_serializing_if = "Option:: is_none")]
    pub is_series: Option<bool>,
    #[serde(rename = "IsNews", skip_serializing_if = "Option:: is_none")]
    pub is_news: Option<bool>,
    #[serde(rename = "IsKids", skip_serializing_if = "Option:: is_none")]
    pub is_kids: Option<bool>,
    #[serde(rename = "IsSports", skip_serializing_if = "Option:: is_none")]
    pub is_sports: Option<bool>,
    #[serde(rename = "StartIndex", skip_serializing_if = "Option:: is_none")]
    pub start_index: Option<f32>,
    #[serde(rename = "Limit", skip_serializing_if = "Option:: is_none")]
    pub limit: Option<f32>,
    #[serde(rename = "SortBy", skip_serializing_if = "Option:: is_none")]
    pub sort_by: Option<Vec<String>>,
    #[serde(rename = "SortOrder", skip_serializing_if = "Option:: is_none")]
    pub sort_order: Option<Vec<SortOrder>>,
    #[serde(rename = "Genres", skip_serializing_if = "Option:: is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "GenreIds", skip_serializing_if = "Option:: is_none")]
    pub genre_ids: Option<Vec<String>>,
    #[serde(rename = "EnableImages", skip_serializing_if = "Option:: is_none")]
    pub enable_images: Option<bool>,
    #[serde(
        rename = "EnableTotalRecordCount",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_total_record_count: Option<bool>,
    #[serde(rename = "ImageTypeLimit", skip_serializing_if = "Option:: is_none")]
    pub image_type_limit: Option<f32>,
    #[serde(rename = "EnableImageTypes", skip_serializing_if = "Option:: is_none")]
    pub enable_image_types: Option<Vec<ImageType>>,
    #[serde(rename = "EnableUserData", skip_serializing_if = "Option:: is_none")]
    pub enable_user_data: Option<bool>,
    #[serde(rename = "SeriesTimerId", skip_serializing_if = "Option:: is_none")]
    pub series_timer_id: Option<String>,
    #[serde(rename = "LibrarySeriesId", skip_serializing_if = "Option:: is_none")]
    pub library_series_id: Option<String>,
    #[serde(rename = "Fields", skip_serializing_if = "Option:: is_none")]
    pub fields: Option<Vec<ItemFields>>,
}
