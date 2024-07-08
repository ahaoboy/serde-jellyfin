#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadSubtitleDto {
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "IsForced")]
    pub is_forced: bool,
    #[serde(rename = "Data")]
    pub data: String,
}
