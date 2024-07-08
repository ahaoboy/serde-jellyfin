use crate::image_saving_convention::ImageSavingConvention;
use crate::metadata_options::MetadataOptions;
use crate::name_value_pair::NameValuePair;
use crate::path_substitution::PathSubstitution;
use crate::repository_info::RepositoryInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerConfiguration {
    #[serde(
        rename = "LogFileRetentionDays",
        skip_serializing_if = "Option:: is_none"
    )]
    pub log_file_retention_days: Option<f32>,
    #[serde(
        rename = "IsStartupWizardCompleted",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_startup_wizard_completed: Option<bool>,
    #[serde(rename = "CachePath", skip_serializing_if = "Option:: is_none")]
    pub cache_path: Option<String>,
    #[serde(rename = "PreviousVersion", skip_serializing_if = "Option:: is_none")]
    pub previous_version: Option<String>,
    #[serde(
        rename = "PreviousVersionStr",
        skip_serializing_if = "Option:: is_none"
    )]
    pub previous_version_str: Option<String>,
    #[serde(rename = "EnableMetrics", skip_serializing_if = "Option:: is_none")]
    pub enable_metrics: Option<bool>,
    #[serde(
        rename = "EnableNormalizedItemByNameIds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_normalized_item_by_name_ids: Option<bool>,
    #[serde(rename = "IsPortAuthorized", skip_serializing_if = "Option:: is_none")]
    pub is_port_authorized: Option<bool>,
    #[serde(
        rename = "QuickConnectAvailable",
        skip_serializing_if = "Option:: is_none"
    )]
    pub quick_connect_available: Option<bool>,
    #[serde(
        rename = "EnableCaseSensitiveItemIds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_case_sensitive_item_ids: Option<bool>,
    #[serde(
        rename = "DisableLiveTvChannelUserDataName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub disable_live_tv_channel_user_data_name: Option<bool>,
    #[serde(rename = "MetadataPath", skip_serializing_if = "Option:: is_none")]
    pub metadata_path: Option<String>,
    #[serde(
        rename = "MetadataNetworkPath",
        skip_serializing_if = "Option:: is_none"
    )]
    pub metadata_network_path: Option<String>,
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
        rename = "SortReplaceCharacters",
        skip_serializing_if = "Option:: is_none"
    )]
    pub sort_replace_characters: Option<Vec<String>>,
    #[serde(
        rename = "SortRemoveCharacters",
        skip_serializing_if = "Option:: is_none"
    )]
    pub sort_remove_characters: Option<Vec<String>>,
    #[serde(rename = "SortRemoveWords", skip_serializing_if = "Option:: is_none")]
    pub sort_remove_words: Option<Vec<String>>,
    #[serde(rename = "MinResumePct", skip_serializing_if = "Option:: is_none")]
    pub min_resume_pct: Option<f32>,
    #[serde(rename = "MaxResumePct", skip_serializing_if = "Option:: is_none")]
    pub max_resume_pct: Option<f32>,
    #[serde(
        rename = "MinResumeDurationSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub min_resume_duration_seconds: Option<f32>,
    #[serde(
        rename = "MinAudiobookResume",
        skip_serializing_if = "Option:: is_none"
    )]
    pub min_audiobook_resume: Option<f32>,
    #[serde(
        rename = "MaxAudiobookResume",
        skip_serializing_if = "Option:: is_none"
    )]
    pub max_audiobook_resume: Option<f32>,
    #[serde(
        rename = "LibraryMonitorDelay",
        skip_serializing_if = "Option:: is_none"
    )]
    pub library_monitor_delay: Option<f32>,
    #[serde(
        rename = "ImageSavingConvention",
        skip_serializing_if = "Option:: is_none"
    )]
    pub image_saving_convention: Option<ImageSavingConvention>,
    #[serde(rename = "MetadataOptions", skip_serializing_if = "Option:: is_none")]
    pub metadata_options: Option<Vec<MetadataOptions>>,
    #[serde(
        rename = "SkipDeserializationForBasicTypes",
        skip_serializing_if = "Option:: is_none"
    )]
    pub skip_deserialization_for_basic_types: Option<bool>,
    #[serde(rename = "ServerName", skip_serializing_if = "Option:: is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "UICulture", skip_serializing_if = "Option:: is_none")]
    pub ui_culture: Option<String>,
    #[serde(
        rename = "SaveMetadataHidden",
        skip_serializing_if = "Option:: is_none"
    )]
    pub save_metadata_hidden: Option<bool>,
    #[serde(rename = "ContentTypes", skip_serializing_if = "Option:: is_none")]
    pub content_types: Option<Vec<NameValuePair>>,
    #[serde(
        rename = "RemoteClientBitrateLimit",
        skip_serializing_if = "Option:: is_none"
    )]
    pub remote_client_bitrate_limit: Option<f32>,
    #[serde(rename = "EnableFolderView", skip_serializing_if = "Option:: is_none")]
    pub enable_folder_view: Option<bool>,
    #[serde(
        rename = "EnableGroupingIntoCollections",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_grouping_into_collections: Option<bool>,
    #[serde(
        rename = "DisplaySpecialsWithinSeasons",
        skip_serializing_if = "Option:: is_none"
    )]
    pub display_specials_within_seasons: Option<bool>,
    #[serde(rename = "CodecsUsed", skip_serializing_if = "Option:: is_none")]
    pub codecs_used: Option<Vec<String>>,
    #[serde(
        rename = "PluginRepositories",
        skip_serializing_if = "Option:: is_none"
    )]
    pub plugin_repositories: Option<Vec<RepositoryInfo>>,
    #[serde(
        rename = "EnableExternalContentInSuggestions",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_external_content_in_suggestions: Option<bool>,
    #[serde(
        rename = "ImageExtractionTimeoutMs",
        skip_serializing_if = "Option:: is_none"
    )]
    pub image_extraction_timeout_ms: Option<f32>,
    #[serde(rename = "PathSubstitutions", skip_serializing_if = "Option:: is_none")]
    pub path_substitutions: Option<Vec<PathSubstitution>>,
    #[serde(
        rename = "EnableSlowResponseWarning",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_slow_response_warning: Option<bool>,
    #[serde(
        rename = "SlowResponseThresholdMs",
        skip_serializing_if = "Option:: is_none"
    )]
    pub slow_response_threshold_ms: Option<f32>,
    #[serde(rename = "CorsHosts", skip_serializing_if = "Option:: is_none")]
    pub cors_hosts: Option<Vec<String>>,
    #[serde(
        rename = "ActivityLogRetentionDays",
        skip_serializing_if = "Option:: is_none"
    )]
    pub activity_log_retention_days: Option<f32>,
    #[serde(
        rename = "LibraryScanFanoutConcurrency",
        skip_serializing_if = "Option:: is_none"
    )]
    pub library_scan_fanout_concurrency: Option<f32>,
    #[serde(
        rename = "LibraryMetadataRefreshConcurrency",
        skip_serializing_if = "Option:: is_none"
    )]
    pub library_metadata_refresh_concurrency: Option<f32>,
    #[serde(rename = "RemoveOldPlugins", skip_serializing_if = "Option:: is_none")]
    pub remove_old_plugins: Option<bool>,
    #[serde(
        rename = "AllowClientLogUpload",
        skip_serializing_if = "Option:: is_none"
    )]
    pub allow_client_log_upload: Option<bool>,
}
