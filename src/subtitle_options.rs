#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubtitleOptions {
    #[serde(
        rename = "SkipIfEmbeddedSubtitlesPresent",
        skip_serializing_if = "Option:: is_none"
    )]
    pub skip_if_embedded_subtitles_present: Option<bool>,
    #[serde(
        rename = "SkipIfAudioTrackMatches",
        skip_serializing_if = "Option:: is_none"
    )]
    pub skip_if_audio_track_matches: Option<bool>,
    #[serde(rename = "DownloadLanguages", skip_serializing_if = "Option:: is_none")]
    pub download_languages: Option<Vec<String>>,
    #[serde(
        rename = "DownloadMovieSubtitles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub download_movie_subtitles: Option<bool>,
    #[serde(
        rename = "DownloadEpisodeSubtitles",
        skip_serializing_if = "Option:: is_none"
    )]
    pub download_episode_subtitles: Option<bool>,
    #[serde(
        rename = "OpenSubtitlesUsername",
        skip_serializing_if = "Option:: is_none"
    )]
    pub open_subtitles_username: Option<String>,
    #[serde(
        rename = "OpenSubtitlesPasswordHash",
        skip_serializing_if = "Option:: is_none"
    )]
    pub open_subtitles_password_hash: Option<String>,
    #[serde(
        rename = "IsOpenSubtitleVipAccount",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_open_subtitle_vip_account: Option<bool>,
    #[serde(
        rename = "RequirePerfectMatch",
        skip_serializing_if = "Option:: is_none"
    )]
    pub require_perfect_match: Option<bool>,
}
