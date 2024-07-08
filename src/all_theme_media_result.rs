use crate::theme_media_result::ThemeMediaResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllThemeMediaResult {
    #[serde(rename = "ThemeVideosResult", skip_serializing_if = "Option:: is_none")]
    pub theme_videos_result: Option<ThemeMediaResult>,
    #[serde(rename = "ThemeSongsResult", skip_serializing_if = "Option:: is_none")]
    pub theme_songs_result: Option<ThemeMediaResult>,
    #[serde(
        rename = "SoundtrackSongsResult",
        skip_serializing_if = "Option:: is_none"
    )]
    pub soundtrack_songs_result: Option<ThemeMediaResult>,
}
