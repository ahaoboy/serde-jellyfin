use crate::media_path_info::MediaPathInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateMediaPathRequestDto {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PathInfo")]
    pub path_info: MediaPathInfo,
}
