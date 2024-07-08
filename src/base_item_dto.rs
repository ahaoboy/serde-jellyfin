use crate::base_item_dto_image_blur_hashes::BaseItemDtoImageBlurHashes;
use crate::base_item_kind::BaseItemKind;
use crate::base_item_person::BaseItemPerson;
use crate::channel_type::ChannelType;
use crate::chapter_info::ChapterInfo;
use crate::day_of_week::DayOfWeek;
use crate::external_url::ExternalUrl;
use crate::image_orientation::ImageOrientation;
use crate::media_source_info::MediaSourceInfo;
use crate::media_stream::MediaStream;
use crate::media_url::MediaUrl;
use crate::name_guid_pair::NameGuidPair;
use crate::user_item_data_dto::UserItemDataDto;
use crate::IsoType;
use crate::LocationType;
use crate::MetadataField;
use crate::PlayAccess;
use crate::ProgramAudio;
use crate::Video3DFormat;
use crate::VideoType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseItemDto {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "OriginalTitle", skip_serializing_if = "Option:: is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "ServerId", skip_serializing_if = "Option:: is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Etag", skip_serializing_if = "Option:: is_none")]
    pub etag: Option<String>,
    #[serde(rename = "SourceType", skip_serializing_if = "Option:: is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "DateCreated", skip_serializing_if = "Option:: is_none")]
    pub date_created: Option<String>,
    #[serde(
        rename = "DateLastMediaAdded",
        skip_serializing_if = "Option:: is_none"
    )]
    pub date_last_media_added: Option<String>,
    #[serde(rename = "ExtraType", skip_serializing_if = "Option:: is_none")]
    pub extra_type: Option<String>,
    #[serde(
        rename = "AirsBeforeSeasonNumber",
        skip_serializing_if = "Option:: is_none"
    )]
    pub airs_before_season_number: Option<f32>,
    #[serde(
        rename = "AirsAfterSeasonNumber",
        skip_serializing_if = "Option:: is_none"
    )]
    pub airs_after_season_number: Option<f32>,
    #[serde(
        rename = "AirsBeforeEpisodeNumber",
        skip_serializing_if = "Option:: is_none"
    )]
    pub airs_before_episode_number: Option<f32>,
    #[serde(rename = "CanDelete", skip_serializing_if = "Option:: is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename = "CanDownload", skip_serializing_if = "Option:: is_none")]
    pub can_download: Option<bool>,
    #[serde(rename = "HasSubtitles", skip_serializing_if = "Option:: is_none")]
    pub has_subtitles: Option<bool>,
    #[serde(
        rename = "PreferredMetadataLanguage",
        skip_serializing_if = "Option:: is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[serde(
        rename = "PreferredMetadataCountryCode",
        skip_serializing_if = "Option:: is_none"
    )]
    pub preferred_metadata_country_code: Option<String>,
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option:: is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
    #[serde(rename = "SortName", skip_serializing_if = "Option:: is_none")]
    pub sort_name: Option<String>,
    #[serde(rename = "ForcedSortName", skip_serializing_if = "Option:: is_none")]
    pub forced_sort_name: Option<String>,
    #[serde(rename = "Video3DFormat", skip_serializing_if = "Option:: is_none")]
    pub video3_d_format: Option<Video3DFormat>,
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option:: is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename = "ExternalUrls", skip_serializing_if = "Option:: is_none")]
    pub external_urls: Option<Vec<ExternalUrl>>,
    #[serde(rename = "MediaSources", skip_serializing_if = "Option:: is_none")]
    pub media_sources: Option<Vec<MediaSourceInfo>>,
    #[serde(rename = "CriticRating", skip_serializing_if = "Option:: is_none")]
    pub critic_rating: Option<f32>,
    #[serde(
        rename = "ProductionLocations",
        skip_serializing_if = "Option:: is_none"
    )]
    pub production_locations: Option<Vec<String>>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "EnableMediaSourceDisplay",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_media_source_display: Option<bool>,
    #[serde(rename = "OfficialRating", skip_serializing_if = "Option:: is_none")]
    pub official_rating: Option<String>,
    #[serde(rename = "CustomRating", skip_serializing_if = "Option:: is_none")]
    pub custom_rating: Option<String>,
    #[serde(rename = "ChannelId", skip_serializing_if = "Option:: is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName", skip_serializing_if = "Option:: is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "Overview", skip_serializing_if = "Option:: is_none")]
    pub overview: Option<String>,
    #[serde(rename = "Taglines", skip_serializing_if = "Option:: is_none")]
    pub taglines: Option<Vec<String>>,
    #[serde(rename = "Genres", skip_serializing_if = "Option:: is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "CommunityRating", skip_serializing_if = "Option:: is_none")]
    pub community_rating: Option<f32>,
    #[serde(
        rename = "CumulativeRunTimeTicks",
        skip_serializing_if = "Option:: is_none"
    )]
    pub cumulative_run_time_ticks: Option<f32>,
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option:: is_none")]
    pub run_time_ticks: Option<f32>,
    #[serde(rename = "PlayAccess", skip_serializing_if = "Option:: is_none")]
    pub play_access: Option<PlayAccess>,
    #[serde(rename = "AspectRatio", skip_serializing_if = "Option:: is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option:: is_none")]
    pub production_year: Option<f32>,
    #[serde(rename = "IsPlaceHolder", skip_serializing_if = "Option:: is_none")]
    pub is_place_holder: Option<bool>,
    #[serde(rename = "Number", skip_serializing_if = "Option:: is_none")]
    pub number: Option<String>,
    #[serde(rename = "ChannelNumber", skip_serializing_if = "Option:: is_none")]
    pub channel_number: Option<String>,
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option:: is_none")]
    pub index_number: Option<f32>,
    #[serde(rename = "IndexNumberEnd", skip_serializing_if = "Option:: is_none")]
    pub index_number_end: Option<f32>,
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option:: is_none")]
    pub parent_index_number: Option<f32>,
    #[serde(rename = "RemoteTrailers", skip_serializing_if = "Option:: is_none")]
    pub remote_trailers: Option<Vec<MediaUrl>>,
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option:: is_none")]
    pub provider_ids: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IsHD", skip_serializing_if = "Option:: is_none")]
    pub is_hd: Option<bool>,
    #[serde(rename = "IsFolder", skip_serializing_if = "Option:: is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename = "ParentId", skip_serializing_if = "Option:: is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<BaseItemKind>,
    #[serde(rename = "People", skip_serializing_if = "Option:: is_none")]
    pub people: Option<Vec<BaseItemPerson>>,
    #[serde(rename = "Studios", skip_serializing_if = "Option:: is_none")]
    pub studios: Option<Vec<NameGuidPair>>,
    #[serde(rename = "GenreItems", skip_serializing_if = "Option:: is_none")]
    pub genre_items: Option<Vec<NameGuidPair>>,
    #[serde(rename = "ParentLogoItemId", skip_serializing_if = "Option:: is_none")]
    pub parent_logo_item_id: Option<String>,
    #[serde(
        rename = "ParentBackdropItemId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_backdrop_item_id: Option<String>,
    #[serde(
        rename = "ParentBackdropImageTags",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_backdrop_image_tags: Option<Vec<String>>,
    #[serde(rename = "LocalTrailerCount", skip_serializing_if = "Option:: is_none")]
    pub local_trailer_count: Option<f32>,
    #[serde(rename = "UserData", skip_serializing_if = "Option:: is_none")]
    pub user_data: Option<UserItemDataDto>,
    #[serde(
        rename = "RecursiveItemCount",
        skip_serializing_if = "Option:: is_none"
    )]
    pub recursive_item_count: Option<f32>,
    #[serde(rename = "ChildCount", skip_serializing_if = "Option:: is_none")]
    pub child_count: Option<f32>,
    #[serde(rename = "SeriesName", skip_serializing_if = "Option:: is_none")]
    pub series_name: Option<String>,
    #[serde(rename = "SeriesId", skip_serializing_if = "Option:: is_none")]
    pub series_id: Option<String>,
    #[serde(rename = "SeasonId", skip_serializing_if = "Option:: is_none")]
    pub season_id: Option<String>,
    #[serde(
        rename = "SpecialFeatureCount",
        skip_serializing_if = "Option:: is_none"
    )]
    pub special_feature_count: Option<f32>,
    #[serde(
        rename = "DisplayPreferencesId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub display_preferences_id: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option:: is_none")]
    pub status: Option<String>,
    #[serde(rename = "AirTime", skip_serializing_if = "Option:: is_none")]
    pub air_time: Option<String>,
    #[serde(rename = "AirDays", skip_serializing_if = "Option:: is_none")]
    pub air_days: Option<Vec<DayOfWeek>>,
    #[serde(rename = "Tags", skip_serializing_if = "Option:: is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(
        rename = "PrimaryImageAspectRatio",
        skip_serializing_if = "Option:: is_none"
    )]
    pub primary_image_aspect_ratio: Option<f32>,
    #[serde(rename = "Artists", skip_serializing_if = "Option:: is_none")]
    pub artists: Option<Vec<String>>,
    #[serde(rename = "ArtistItems", skip_serializing_if = "Option:: is_none")]
    pub artist_items: Option<Vec<NameGuidPair>>,
    #[serde(rename = "Album", skip_serializing_if = "Option:: is_none")]
    pub album: Option<String>,
    #[serde(rename = "CollectionType", skip_serializing_if = "Option:: is_none")]
    pub collection_type: Option<String>,
    #[serde(rename = "DisplayOrder", skip_serializing_if = "Option:: is_none")]
    pub display_order: Option<String>,
    #[serde(rename = "AlbumId", skip_serializing_if = "Option:: is_none")]
    pub album_id: Option<String>,
    #[serde(
        rename = "AlbumPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub album_primary_image_tag: Option<String>,
    #[serde(
        rename = "SeriesPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub series_primary_image_tag: Option<String>,
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option:: is_none")]
    pub album_artist: Option<String>,
    #[serde(rename = "AlbumArtists", skip_serializing_if = "Option:: is_none")]
    pub album_artists: Option<Vec<NameGuidPair>>,
    #[serde(rename = "SeasonName", skip_serializing_if = "Option:: is_none")]
    pub season_name: Option<String>,
    #[serde(rename = "MediaStreams", skip_serializing_if = "Option:: is_none")]
    pub media_streams: Option<Vec<MediaStream>>,
    #[serde(rename = "VideoType", skip_serializing_if = "Option:: is_none")]
    pub video_type: Option<VideoType>,
    #[serde(rename = "PartCount", skip_serializing_if = "Option:: is_none")]
    pub part_count: Option<f32>,
    #[serde(rename = "MediaSourceCount", skip_serializing_if = "Option:: is_none")]
    pub media_source_count: Option<f32>,
    #[serde(rename = "ImageTags", skip_serializing_if = "Option:: is_none")]
    pub image_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "BackdropImageTags", skip_serializing_if = "Option:: is_none")]
    pub backdrop_image_tags: Option<Vec<String>>,
    #[serde(
        rename = "ScreenshotImageTags",
        skip_serializing_if = "Option:: is_none"
    )]
    pub screenshot_image_tags: Option<Vec<String>>,
    #[serde(
        rename = "ParentLogoImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_logo_image_tag: Option<String>,
    #[serde(rename = "ParentArtItemId", skip_serializing_if = "Option:: is_none")]
    pub parent_art_item_id: Option<String>,
    #[serde(rename = "ParentArtImageTag", skip_serializing_if = "Option:: is_none")]
    pub parent_art_image_tag: Option<String>,
    #[serde(
        rename = "SeriesThumbImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub series_thumb_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes", skip_serializing_if = "Option:: is_none")]
    pub image_blur_hashes: Option<BaseItemDtoImageBlurHashes>,
    #[serde(rename = "SeriesStudio", skip_serializing_if = "Option:: is_none")]
    pub series_studio: Option<String>,
    #[serde(rename = "ParentThumbItemId", skip_serializing_if = "Option:: is_none")]
    pub parent_thumb_item_id: Option<String>,
    #[serde(
        rename = "ParentThumbImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_thumb_image_tag: Option<String>,
    #[serde(
        rename = "ParentPrimaryImageItemId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_primary_image_item_id: Option<String>,
    #[serde(
        rename = "ParentPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_primary_image_tag: Option<String>,
    #[serde(rename = "Chapters", skip_serializing_if = "Option:: is_none")]
    pub chapters: Option<Vec<ChapterInfo>>,
    #[serde(rename = "LocationType", skip_serializing_if = "Option:: is_none")]
    pub location_type: Option<LocationType>,
    #[serde(rename = "IsoType", skip_serializing_if = "Option:: is_none")]
    pub iso_type: Option<IsoType>,
    #[serde(rename = "MediaType", skip_serializing_if = "Option:: is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option:: is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "LockedFields", skip_serializing_if = "Option:: is_none")]
    pub locked_fields: Option<Vec<MetadataField>>,
    #[serde(rename = "TrailerCount", skip_serializing_if = "Option:: is_none")]
    pub trailer_count: Option<f32>,
    #[serde(rename = "MovieCount", skip_serializing_if = "Option:: is_none")]
    pub movie_count: Option<f32>,
    #[serde(rename = "SeriesCount", skip_serializing_if = "Option:: is_none")]
    pub series_count: Option<f32>,
    #[serde(rename = "ProgramCount", skip_serializing_if = "Option:: is_none")]
    pub program_count: Option<f32>,
    #[serde(rename = "EpisodeCount", skip_serializing_if = "Option:: is_none")]
    pub episode_count: Option<f32>,
    #[serde(rename = "SongCount", skip_serializing_if = "Option:: is_none")]
    pub song_count: Option<f32>,
    #[serde(rename = "AlbumCount", skip_serializing_if = "Option:: is_none")]
    pub album_count: Option<f32>,
    #[serde(rename = "ArtistCount", skip_serializing_if = "Option:: is_none")]
    pub artist_count: Option<f32>,
    #[serde(rename = "MusicVideoCount", skip_serializing_if = "Option:: is_none")]
    pub music_video_count: Option<f32>,
    #[serde(rename = "LockData", skip_serializing_if = "Option:: is_none")]
    pub lock_data: Option<bool>,
    #[serde(rename = "Width", skip_serializing_if = "Option:: is_none")]
    pub width: Option<f32>,
    #[serde(rename = "Height", skip_serializing_if = "Option:: is_none")]
    pub height: Option<f32>,
    #[serde(rename = "CameraMake", skip_serializing_if = "Option:: is_none")]
    pub camera_make: Option<String>,
    #[serde(rename = "CameraModel", skip_serializing_if = "Option:: is_none")]
    pub camera_model: Option<String>,
    #[serde(rename = "Software", skip_serializing_if = "Option:: is_none")]
    pub software: Option<String>,
    #[serde(rename = "ExposureTime", skip_serializing_if = "Option:: is_none")]
    pub exposure_time: Option<f32>,
    #[serde(rename = "FocalLength", skip_serializing_if = "Option:: is_none")]
    pub focal_length: Option<f32>,
    #[serde(rename = "ImageOrientation", skip_serializing_if = "Option:: is_none")]
    pub image_orientation: Option<ImageOrientation>,
    #[serde(rename = "Aperture", skip_serializing_if = "Option:: is_none")]
    pub aperture: Option<f32>,
    #[serde(rename = "ShutterSpeed", skip_serializing_if = "Option:: is_none")]
    pub shutter_speed: Option<f32>,
    #[serde(rename = "Latitude", skip_serializing_if = "Option:: is_none")]
    pub latitude: Option<f32>,
    #[serde(rename = "Longitude", skip_serializing_if = "Option:: is_none")]
    pub longitude: Option<f32>,
    #[serde(rename = "Altitude", skip_serializing_if = "Option:: is_none")]
    pub altitude: Option<f32>,
    #[serde(rename = "IsoSpeedRating", skip_serializing_if = "Option:: is_none")]
    pub iso_speed_rating: Option<f32>,
    #[serde(rename = "SeriesTimerId", skip_serializing_if = "Option:: is_none")]
    pub series_timer_id: Option<String>,
    #[serde(rename = "ProgramId", skip_serializing_if = "Option:: is_none")]
    pub program_id: Option<String>,
    #[serde(
        rename = "ChannelPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option:: is_none")]
    pub start_date: Option<String>,
    #[serde(
        rename = "CompletionPercentage",
        skip_serializing_if = "Option:: is_none"
    )]
    pub completion_percentage: Option<f32>,
    #[serde(rename = "IsRepeat", skip_serializing_if = "Option:: is_none")]
    pub is_repeat: Option<bool>,
    #[serde(rename = "EpisodeTitle", skip_serializing_if = "Option:: is_none")]
    pub episode_title: Option<String>,
    #[serde(rename = "ChannelType", skip_serializing_if = "Option:: is_none")]
    pub channel_type: Option<ChannelType>,
    #[serde(rename = "Audio", skip_serializing_if = "Option:: is_none")]
    pub audio: Option<ProgramAudio>,
    #[serde(rename = "IsMovie", skip_serializing_if = "Option:: is_none")]
    pub is_movie: Option<bool>,
    #[serde(rename = "IsSports", skip_serializing_if = "Option:: is_none")]
    pub is_sports: Option<bool>,
    #[serde(rename = "IsSeries", skip_serializing_if = "Option:: is_none")]
    pub is_series: Option<bool>,
    #[serde(rename = "IsLive", skip_serializing_if = "Option:: is_none")]
    pub is_live: Option<bool>,
    #[serde(rename = "IsNews", skip_serializing_if = "Option:: is_none")]
    pub is_news: Option<bool>,
    #[serde(rename = "IsKids", skip_serializing_if = "Option:: is_none")]
    pub is_kids: Option<bool>,
    #[serde(rename = "IsPremiere", skip_serializing_if = "Option:: is_none")]
    pub is_premiere: Option<bool>,
    #[serde(rename = "TimerId", skip_serializing_if = "Option:: is_none")]
    pub timer_id: Option<String>,
    #[serde(rename = "CurrentProgram", skip_serializing_if = "Option:: is_none")]
    pub current_program: Option<Box<BaseItemDto>>,
}
