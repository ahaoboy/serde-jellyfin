use crate::embedded_subtitle_options::EmbeddedSubtitleOptions;
use crate::media_path_info::MediaPathInfo;
use crate::type_options::TypeOptions;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryOptions {
    #[serde(rename = "EnablePhotos", skip_serializing_if = "Option:: is_none")]
    pub enable_photos: Option<bool>,
    #[serde(
        rename = "EnableRealtimeMonitor",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_realtime_monitor: Option<bool>,
    #[serde(
        rename = "EnableChapterImageExtraction",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_chapter_image_extraction: Option<bool>,
    #[serde(
        rename = "ExtractChapterImagesDuringLibraryScan",
        skip_serializing_if = "Option:: is_none"
    )]
    pub extract_chapter_images_during_library_scan: Option<bool>,
    #[serde(rename = "PathInfos", skip_serializing_if = "Option:: is_none")]
    pub path_infos: Option<Vec<MediaPathInfo>>,
    #[serde(rename = "SaveLocalMetadata", skip_serializing_if = "Option:: is_none")]
    pub save_local_metadata: Option<bool>,
    #[serde(
        rename = "EnableInternetProviders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_internet_providers: Option<bool>,
    #[serde(
        rename = "EnableAutomaticSeriesGrouping",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_automatic_series_grouping: Option<bool>,
    #[serde(
        rename = "EnableEmbeddedTitles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_embedded_titles: Option<bool>,
    #[serde(
        rename = "EnableEmbeddedEpisodeInfos",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_embedded_episode_infos: Option<bool>,
    #[serde(
        rename = "AutomaticRefreshIntervalDays",
        skip_serializing_if = "Option:: is_none"
    )]
    pub automatic_refresh_interval_days: Option<f32>,
    #[serde(
        rename = "PreferredMetadataLanguage",
        skip_serializing_if = "Option:: is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[serde(
        rename = "MetadataCountryCode",
        skip_serializing_if = "Option:: is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[serde(
        rename = "SeasonZeroDisplayName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub season_zero_display_name: Option<String>,
    #[serde(rename = "MetadataSavers", skip_serializing_if = "Option:: is_none")]
    pub metadata_savers: Option<Vec<String>>,
    #[serde(
        rename = "DisabledLocalMetadataReaders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub disabled_local_metadata_readers: Option<Vec<String>>,
    #[serde(
        rename = "LocalMetadataReaderOrder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub local_metadata_reader_order: Option<Vec<String>>,
    #[serde(
        rename = "DisabledSubtitleFetchers",
        skip_serializing_if = "Option:: is_none"
    )]
    pub disabled_subtitle_fetchers: Option<Vec<String>>,
    #[serde(
        rename = "SubtitleFetcherOrder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub subtitle_fetcher_order: Option<Vec<String>>,
    #[serde(
        rename = "SkipSubtitlesIfEmbeddedSubtitlesPresent",
        skip_serializing_if = "Option:: is_none"
    )]
    pub skip_subtitles_if_embedded_subtitles_present: Option<bool>,
    #[serde(
        rename = "SkipSubtitlesIfAudioTrackMatches",
        skip_serializing_if = "Option:: is_none"
    )]
    pub skip_subtitles_if_audio_track_matches: Option<bool>,
    #[serde(
        rename = "SubtitleDownloadLanguages",
        skip_serializing_if = "Option:: is_none"
    )]
    pub subtitle_download_languages: Option<Vec<String>>,
    #[serde(
        rename = "RequirePerfectSubtitleMatch",
        skip_serializing_if = "Option:: is_none"
    )]
    pub require_perfect_subtitle_match: Option<bool>,
    #[serde(
        rename = "SaveSubtitlesWithMedia",
        skip_serializing_if = "Option:: is_none"
    )]
    pub save_subtitles_with_media: Option<bool>,
    #[serde(
        rename = "AutomaticallyAddToCollection",
        skip_serializing_if = "Option:: is_none"
    )]
    pub automatically_add_to_collection: Option<bool>,
    #[serde(
        rename = "AllowEmbeddedSubtitles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub allow_embedded_subtitles: Option<EmbeddedSubtitleOptions>,
    #[serde(rename = "TypeOptions", skip_serializing_if = "Option:: is_none")]
    pub type_options: Option<Vec<TypeOptions>>,
}
