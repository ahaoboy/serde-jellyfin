#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemCounts {
    #[serde(rename = "MovieCount", skip_serializing_if = "Option:: is_none")]
    pub movie_count: Option<f32>,
    #[serde(rename = "SeriesCount", skip_serializing_if = "Option:: is_none")]
    pub series_count: Option<f32>,
    #[serde(rename = "EpisodeCount", skip_serializing_if = "Option:: is_none")]
    pub episode_count: Option<f32>,
    #[serde(rename = "ArtistCount", skip_serializing_if = "Option:: is_none")]
    pub artist_count: Option<f32>,
    #[serde(rename = "ProgramCount", skip_serializing_if = "Option:: is_none")]
    pub program_count: Option<f32>,
    #[serde(rename = "TrailerCount", skip_serializing_if = "Option:: is_none")]
    pub trailer_count: Option<f32>,
    #[serde(rename = "SongCount", skip_serializing_if = "Option:: is_none")]
    pub song_count: Option<f32>,
    #[serde(rename = "AlbumCount", skip_serializing_if = "Option:: is_none")]
    pub album_count: Option<f32>,
    #[serde(rename = "MusicVideoCount", skip_serializing_if = "Option:: is_none")]
    pub music_video_count: Option<f32>,
    #[serde(rename = "BoxSetCount", skip_serializing_if = "Option:: is_none")]
    pub box_set_count: Option<f32>,
    #[serde(rename = "BookCount", skip_serializing_if = "Option:: is_none")]
    pub book_count: Option<f32>,
    #[serde(rename = "ItemCount", skip_serializing_if = "Option:: is_none")]
    pub item_count: Option<f32>,
}
