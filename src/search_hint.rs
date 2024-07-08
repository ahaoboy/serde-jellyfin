#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchHint {
    #[serde(rename = "ItemId", skip_serializing_if = "Option:: is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "MatchedTerm", skip_serializing_if = "Option:: is_none")]
    pub matched_term: Option<String>,
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option:: is_none")]
    pub index_number: Option<f32>,
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option:: is_none")]
    pub production_year: Option<f32>,
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option:: is_none")]
    pub parent_index_number: Option<f32>,
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option:: is_none")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "ThumbImageTag", skip_serializing_if = "Option:: is_none")]
    pub thumb_image_tag: Option<String>,
    #[serde(rename = "ThumbImageItemId", skip_serializing_if = "Option:: is_none")]
    pub thumb_image_item_id: Option<String>,
    #[serde(rename = "BackdropImageTag", skip_serializing_if = "Option:: is_none")]
    pub backdrop_image_tag: Option<String>,
    #[serde(
        rename = "BackdropImageItemId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub backdrop_image_item_id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "IsFolder", skip_serializing_if = "Option:: is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option:: is_none")]
    pub run_time_ticks: Option<f32>,
    #[serde(rename = "MediaType", skip_serializing_if = "Option:: is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option:: is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option:: is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Series", skip_serializing_if = "Option:: is_none")]
    pub series: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option:: is_none")]
    pub status: Option<String>,
    #[serde(rename = "Album", skip_serializing_if = "Option:: is_none")]
    pub album: Option<String>,
    #[serde(rename = "AlbumId", skip_serializing_if = "Option:: is_none")]
    pub album_id: Option<String>,
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option:: is_none")]
    pub album_artist: Option<String>,
    #[serde(rename = "Artists", skip_serializing_if = "Option:: is_none")]
    pub artists: Option<Vec<String>>,
    #[serde(rename = "SongCount", skip_serializing_if = "Option:: is_none")]
    pub song_count: Option<f32>,
    #[serde(rename = "EpisodeCount", skip_serializing_if = "Option:: is_none")]
    pub episode_count: Option<f32>,
    #[serde(rename = "ChannelId", skip_serializing_if = "Option:: is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName", skip_serializing_if = "Option:: is_none")]
    pub channel_name: Option<String>,
    #[serde(
        rename = "PrimaryImageAspectRatio",
        skip_serializing_if = "Option:: is_none"
    )]
    pub primary_image_aspect_ratio: Option<f32>,
}
