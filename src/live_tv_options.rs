use crate::listings_provider_info::ListingsProviderInfo;
use crate::tuner_host_info::TunerHostInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LiveTvOptions {
    #[serde(rename = "GuideDays", skip_serializing_if = "Option:: is_none")]
    pub guide_days: Option<f32>,
    #[serde(rename = "RecordingPath", skip_serializing_if = "Option:: is_none")]
    pub recording_path: Option<String>,
    #[serde(
        rename = "MovieRecordingPath",
        skip_serializing_if = "Option:: is_none"
    )]
    pub movie_recording_path: Option<String>,
    #[serde(
        rename = "SeriesRecordingPath",
        skip_serializing_if = "Option:: is_none"
    )]
    pub series_recording_path: Option<String>,
    #[serde(
        rename = "EnableRecordingSubfolders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_recording_subfolders: Option<bool>,
    #[serde(
        rename = "EnableOriginalAudioWithEncodedRecordings",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_original_audio_with_encoded_recordings: Option<bool>,
    #[serde(rename = "TunerHosts", skip_serializing_if = "Option:: is_none")]
    pub tuner_hosts: Option<Vec<TunerHostInfo>>,
    #[serde(rename = "ListingProviders", skip_serializing_if = "Option:: is_none")]
    pub listing_providers: Option<Vec<ListingsProviderInfo>>,
    #[serde(rename = "PrePaddingSeconds", skip_serializing_if = "Option:: is_none")]
    pub pre_padding_seconds: Option<f32>,
    #[serde(
        rename = "PostPaddingSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub post_padding_seconds: Option<f32>,
    #[serde(
        rename = "MediaLocationsCreated",
        skip_serializing_if = "Option:: is_none"
    )]
    pub media_locations_created: Option<Vec<String>>,
    #[serde(
        rename = "RecordingPostProcessor",
        skip_serializing_if = "Option:: is_none"
    )]
    pub recording_post_processor: Option<String>,
    #[serde(
        rename = "RecordingPostProcessorArguments",
        skip_serializing_if = "Option:: is_none"
    )]
    pub recording_post_processor_arguments: Option<String>,
}
