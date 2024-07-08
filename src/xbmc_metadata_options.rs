#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XbmcMetadataOptions {
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "ReleaseDateFormat", skip_serializing_if = "Option:: is_none")]
    pub release_date_format: Option<String>,
    #[serde(
        rename = "SaveImagePathsInNfo",
        skip_serializing_if = "Option:: is_none"
    )]
    pub save_image_paths_in_nfo: Option<bool>,
    #[serde(
        rename = "EnablePathSubstitution",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_path_substitution: Option<bool>,
    #[serde(
        rename = "EnableExtraThumbsDuplication",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_extra_thumbs_duplication: Option<bool>,
}
